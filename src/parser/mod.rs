use pest::{
    iterators::Pair,
    prec_climber::{Assoc, Operator, PrecClimber},
    Parser,
};

use crate::ast::{
    ast::{
        BinaryOp, Block, Decl, DeclKind, Expr, ExprKind, Ident, Lit, LitKind, LogicalOp, Program,
        Stmt, StmtKind, TopLevelDecl, TopLevelDeclKind, Ty, UnaryOp,
    },
    location::Location,
};

pub mod error;
mod grammar;

use self::{
    error::{ParsingError, ParsingErrorKind},
    grammar::{Grammar, Rule},
};

struct ParsingCtx {
    errors: Vec<ParsingError>,
}

impl ParsingCtx {
    fn new() -> Self {
        ParsingCtx { errors: Vec::new() }
    }

    fn is_failed(&self) -> bool {
        !self.errors.is_empty()
    }
}

pub fn parse(input: &str) -> Result<Program, Vec<ParsingError>> {
    match Grammar::parse(Rule::program, input) {
        Ok(pairs) => {
            let start = 0;
            let mut end = 0;

            let mut ctx = ParsingCtx::new();
            let mut decls = Vec::new();

            for pair in pairs {
                match pair.as_rule() {
                    Rule::EOI => end = pair.as_span().end(),
                    Rule::function_declaration => {
                        match parse_top_level_decl(&mut ctx, pair) {
                            Ok(decl) => decls.push(decl),
                            Err(err) => ctx.errors.push(err),
                        };
                    }
                    _ => unreachable!("Unexpected top level declaration {:?}", pair),
                };
            }

            if ctx.is_failed() {
                Err(ctx.errors)
            } else {
                Ok(Program {
                    decls,
                    location: Location::new(start, end),
                })
            }
        }
        Err(err) => Err(vec![ParsingError::from(err)]),
    }
}

fn parse_top_level_decl(
    ctx: &mut ParsingCtx,
    pair: Pair<Rule>,
) -> Result<TopLevelDecl, ParsingError> {
    let location = Location::from(&pair);

    match pair.as_rule() {
        Rule::function_declaration => {
            let mut inner = pair.into_inner();

            let ident = parse_ident(inner.next().unwrap())?;

            let mut params = Vec::new();
            let mut parameter_pairs = inner.next().unwrap().into_inner();
            while let (Some(name), Some(ty)) = (parameter_pairs.next(), parameter_pairs.next()) {
                params.push((parse_ident(name)?, parse_ty(ty)?));
            }

            let return_ty = parse_ty(inner.next().unwrap())?;

            let body = parse_block(ctx, inner.next().unwrap());

            Ok(TopLevelDecl {
                kind: TopLevelDeclKind::Fn(ident, params, return_ty, body),
                location,
            })
        }
        _ => unreachable!("Unexpected top level declaration {:?}", pair),
    }
}

fn parse_stmt(ctx: &mut ParsingCtx, pair: Pair<Rule>) -> Option<Stmt> {
    let parse_stmt_inner = || {
        let location = Location::from(&pair);

        match pair.as_rule() {
            Rule::variable_declaration => {
                let mut inner = pair.into_inner();

                let ident = parse_ident(inner.next().unwrap())?;
                let mut ty: Option<Ty> = None;
                let mut init: Option<Expr> = None;

                for inner_pair in inner {
                    match inner_pair.as_rule() {
                        Rule::ty => ty = Some(parse_ty(inner_pair)?),
                        Rule::expression => init = Some(parse_expr(ctx, inner_pair)?),
                        _ => {
                            unreachable!("Unexpected variable declaration {:?}", inner_pair)
                        }
                    }
                }

                Ok(Stmt {
                    kind: StmtKind::Decl(Decl {
                        kind: DeclKind::Var(ident, ty, init),
                        location,
                    }),
                    location,
                })
            }
            Rule::return_statement => {
                let mut inner = pair.into_inner();

                let expr = match inner.next() {
                    Some(expr) => Some(parse_expr(ctx, expr)?),
                    None => None,
                };

                Ok(Stmt {
                    kind: StmtKind::Ret(expr),
                    location,
                })
            }
            Rule::expression_statement => {
                let mut inner = pair.into_inner();

                let expr = parse_expr(ctx, inner.next().unwrap())?;

                Ok(Stmt {
                    kind: StmtKind::Expr(expr),
                    location,
                })
            }
            Rule::block => {
                let block = parse_block(ctx, pair);

                Ok(Stmt {
                    kind: StmtKind::Block(block),
                    location,
                })
            }
            _ => unreachable!("Unexpected declaration {:?}", pair),
        }
    };

    match parse_stmt_inner() {
        Ok(stmt) => Some(stmt),
        Err(err) => {
            ctx.errors.push(err);
            None
        }
    }
}

fn parse_block(ctx: &mut ParsingCtx, pair: Pair<Rule>) -> Block {
    let location = Location::from(&pair);

    match pair.as_rule() {
        Rule::block => {
            let mut stmts = Vec::new();

            for pair in pair.into_inner() {
                if let Some(stmt) = parse_stmt(ctx, pair) {
                    stmts.push(stmt);
                }
            }

            Block { stmts, location }
        }
        _ => unreachable!("Unexpected declaration {:?}", pair),
    }
}

lazy_static! {
    static ref PREC_LOGICAL_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![Operator::new(or, Left), Operator::new(and, Left)])
    };
    static ref PREC_BINARY_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(equal_equal, Left) | Operator::new(bang_equal, Left),
            Operator::new(greater, Left)
                | Operator::new(greater_equal, Left)
                | Operator::new(less, Left)
                | Operator::new(less_equal, Left),
            Operator::new(plus, Left) | Operator::new(minus, Left),
            Operator::new(star, Left) | Operator::new(slash, Left),
        ])
    };
}

fn parse_expr(ctx: &mut ParsingCtx, pair: Pair<Rule>) -> Result<Expr, ParsingError> {
    let location = Location::from(&pair);

    match pair.as_rule() {
        Rule::expression => {
            let inner = pair.into_inner().next().unwrap();
            parse_expr(ctx, inner)
        }

        Rule::logical => {
            let inner = pair.into_inner();
            PREC_LOGICAL_CLIMBER.climb(
                inner,
                |pair| parse_expr(ctx, pair),
                |lhs_res, op, rhs_res| {
                    let lhs = lhs_res?;
                    let rhs = rhs_res?;

                    let location = Location::new(lhs.location.start(), rhs.location.end());

                    Ok(Expr {
                        kind: ExprKind::Logical(parse_logical_op(op), Box::new(lhs), Box::new(rhs)),
                        location,
                    })
                },
            )
        }

        Rule::binary => {
            let inner = pair.into_inner();
            PREC_BINARY_CLIMBER.climb(
                inner,
                |pair| parse_expr(ctx, pair),
                |lhs_res, op, rhs_res| {
                    let lhs = lhs_res?;
                    let rhs = rhs_res?;

                    let location = Location::new(lhs.location.start(), rhs.location.end());

                    Ok(Expr {
                        kind: ExprKind::Binary(parse_binary_op(op), Box::new(lhs), Box::new(rhs)),
                        location,
                    })
                },
            )
        }

        Rule::unary => {
            let mut inner = pair.into_inner();
            let next = inner.next().unwrap();

            match next.as_rule() {
                Rule::unary_operator => {
                    let start = next.as_span().start();
                    let op = parse_unary_op(next);
                    let expression = parse_expr(ctx, inner.next().unwrap())?;

                    let location = Location::new(start, expression.location.end());

                    Ok(Expr {
                        kind: ExprKind::Unary(op, Box::new(expression)),
                        location,
                    })
                }
                _ => parse_expr(ctx, next),
            }
        }

        Rule::call => {
            let mut inner = pair.into_inner();

            let mut expr = parse_expr(ctx, inner.next().unwrap())?;

            for pair in inner {
                let location = Location::new(expr.location.start, pair.as_span().end());

                let mut args = Vec::new();
                for inner in pair.into_inner() {
                    let arg = parse_expr(ctx, inner)?;
                    args.push(Box::new(arg));
                }

                expr = Expr {
                    kind: ExprKind::Call(Box::new(expr), args),
                    location,
                }
            }

            Ok(expr)
        }

        Rule::identifier => {
            let ident = parse_ident(pair)?;
            Ok(Expr {
                kind: ExprKind::Ident(ident),
                location,
            })
        }
        Rule::integer | Rule::float | Rule::boolean => {
            let lit = parse_lit(pair)?;
            Ok(Expr {
                kind: ExprKind::Lit(lit),
                location,
            })
        }
        _ => unreachable!("Unexpected expression {:?}", pair),
    }
}

fn parse_logical_op(pair: Pair<Rule>) -> LogicalOp {
    match pair.as_rule() {
        Rule::and => LogicalOp::And,
        Rule::or => LogicalOp::Or,
        _ => unreachable!("Invalid logical operator {:?}", pair),
    }
}

fn parse_binary_op(pair: Pair<Rule>) -> BinaryOp {
    match pair.as_rule() {
        Rule::plus => BinaryOp::Add,
        Rule::minus => BinaryOp::Subtract,
        Rule::star => BinaryOp::Multiply,
        Rule::slash => BinaryOp::Divide,
        Rule::greater => BinaryOp::Greater,
        Rule::greater_equal => BinaryOp::GreaterEqual,
        Rule::less => BinaryOp::Less,
        Rule::less_equal => BinaryOp::LessEqual,
        _ => unreachable!("Invalid binary operator {:?}", pair),
    }
}

fn parse_unary_op(pair: Pair<Rule>) -> UnaryOp {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::bang => UnaryOp::Not,
        Rule::minus => UnaryOp::Minus,
        _ => unreachable!("Invalid unary operator {:?}", inner),
    }
}

fn parse_ident(pair: Pair<Rule>) -> Result<Ident, ParsingError> {
    match pair.as_rule() {
        Rule::identifier => {
            let name = pair.as_str().to_owned();
            let location = Location::from(&pair);

            if is_reserved(&name) {
                Err(ParsingError::new(
                    ParsingErrorKind::ReservedKeyword(name),
                    location,
                ))
            } else {
                Ok(Ident { name, location })
            }
        }
        _ => unreachable!("Unexpected identifier {:?}", pair),
    }
}

fn parse_ty(pair: Pair<Rule>) -> Result<Ty, ParsingError> {
    match pair.as_rule() {
        Rule::ty => {
            let name = pair.as_str().to_owned();
            let location = Location::from(&pair);

            if is_reserved(&name) {
                Err(ParsingError::new(
                    ParsingErrorKind::ReservedKeyword(name),
                    location,
                ))
            } else {
                Ok(Ty { name, location })
            }
        }
        _ => unreachable!("Unexpected type {:?}", pair),
    }
}

fn parse_lit(pair: Pair<Rule>) -> Result<Lit, ParsingError> {
    let location = Location::from(&pair);

    let kind = match pair.as_rule() {
        Rule::boolean => match pair.as_str() {
            "true" => LitKind::Bool(true),
            "false" => LitKind::Bool(false),
            _ => unreachable!("Unexpected boolean value {:?}", pair),
        },
        Rule::integer => {
            let value: u64 = pair.as_str().to_owned().parse().map_err(|_| {
                ParsingError::new(ParsingErrorKind::InvalidInteger(pair.to_string()), location)
            })?;

            LitKind::Int(value)
        }
        Rule::float => {
            let value: f64 = pair.as_str().to_owned().parse().map_err(|_| {
                ParsingError::new(ParsingErrorKind::InvalidFloat(pair.to_string()), location)
            })?;

            LitKind::Float(value)
        }
        _ => unreachable!("Unexpected literal value {:?}", pair),
    };

    Ok(Lit { kind, location })
}

fn is_reserved(name: &str) -> bool {
    matches!(
        name,
        "class" | "else" | "false" | "fn" | "let" | "if" | "true"
    )
}

impl<'a> From<&Pair<'a, Rule>> for Location {
    fn from(pair: &Pair<'a, Rule>) -> Self {
        let span = pair.as_span();
        Location::new(span.start(), span.end())
    }
}
