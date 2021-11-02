use pest::{
    iterators::Pair,
    prec_climber::{Assoc, Operator, PrecClimber},
};

use crate::grammar::Rule;

use super::{Boolean, Float, Identifier, Integer, Locatable, Location, ParsingError};

#[derive(Debug, PartialEq, Eq)]
pub enum LogicalOperator {
    And,
    Or,
}

impl<'a> From<Pair<'a, Rule>> for LogicalOperator {
    fn from(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::and => LogicalOperator::And,
            Rule::or => LogicalOperator::Or,
            _ => unreachable!("Invalid logical operator {:?}", pair),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl<'a> From<Pair<'a, Rule>> for BinaryOperator {
    fn from(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::plus => BinaryOperator::Add,
            Rule::minus => BinaryOperator::Subtract,
            Rule::star => BinaryOperator::Multiply,
            Rule::slash => BinaryOperator::Divide,
            Rule::greater => BinaryOperator::Greater,
            Rule::greater_equal => BinaryOperator::GreaterEqual,
            Rule::less => BinaryOperator::Less,
            Rule::less_equal => BinaryOperator::LessEqual,
            _ => unreachable!("Invalid binary operator {:?}", pair),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOperator {
    Not,
    Minus,
}

impl<'a> From<Pair<'a, Rule>> for UnaryOperator {
    fn from(pair: Pair<'a, Rule>) -> Self {
        let inner = pair.into_inner().next().unwrap();

        match inner.as_rule() {
            Rule::bang => UnaryOperator::Not,
            Rule::minus => UnaryOperator::Minus,
            _ => unreachable!("Invalid unary operator {:?}", inner),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Expression {
    pub expr: Expr,
    pub location: Location,
}

impl Locatable for Expression {
    fn location(&self) -> &Location {
        &self.location
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

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Logical {
        operator: LogicalOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Binary {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        operator: UnaryOperator,
        expression: Box<Expression>,
    },
    Identifier(Identifier),
    Integer(Integer),
    Float(Float),
    Boolean(Boolean),
}

impl<'a> TryFrom<Pair<'a, Rule>> for Expression {
    type Error = ParsingError;

    fn try_from(pair: Pair<'a, Rule>) -> Result<Self, Self::Error> {
        let location = Location::from(&pair);

        match pair.as_rule() {
            Rule::expression => {
                let inner = pair.into_inner().next().unwrap();
                Self::try_from(inner)
            }

            Rule::logical => {
                let inner = pair.into_inner();
                PREC_LOGICAL_CLIMBER.climb(
                    inner,
                    Expression::try_from,
                    |lhs_res: Result<Expression, ParsingError>,
                     op: Pair<Rule>,
                     rhs_res: Result<Expression, ParsingError>| {
                        let lhs = lhs_res?;
                        let rhs = rhs_res?;

                        let location = Location::new(lhs.location.start(), rhs.location.end());
                        Ok(Expression {
                            expr: Expr::Logical {
                                operator: LogicalOperator::from(op),
                                left: Box::new(lhs),
                                right: Box::new(rhs),
                            },
                            location,
                        })
                    },
                )
            }

            Rule::binary => {
                let inner = pair.into_inner();
                PREC_BINARY_CLIMBER.climb(
                    inner,
                    Expression::try_from,
                    |lhs_res: Result<Expression, ParsingError>,
                     op: Pair<Rule>,
                     rhs_res: Result<Expression, ParsingError>| {
                        let lhs = lhs_res?;
                        let rhs = rhs_res?;

                        let location = Location::new(lhs.location.start(), rhs.location.end());
                        Ok(Expression {
                            expr: Expr::Binary {
                                operator: BinaryOperator::from(op),
                                left: Box::new(lhs),
                                right: Box::new(rhs),
                            },
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
                        let operator = UnaryOperator::from(next);
                        let expression = Self::try_from(inner.next().unwrap())?;

                        let location = Location::new(start, expression.location.end());

                        Ok(Expression {
                            expr: Expr::Unary {
                                operator,
                                expression: Box::new(expression),
                            },
                            location,
                        })
                    }
                    _ => Self::try_from(next),
                }
            }

            Rule::identifier => {
                let ident = Identifier::try_from(pair)?;
                Ok(Expression {
                    expr: Expr::Identifier(ident),
                    location,
                })
            }
            Rule::integer => {
                let int = Integer::from(pair);
                Ok(Expression {
                    expr: Expr::Integer(int),
                    location,
                })
            }
            Rule::float => {
                let float = Float::from(pair);
                Ok(Expression {
                    expr: Expr::Float(float),
                    location,
                })
            }
            Rule::boolean => {
                let bool = Boolean::from(pair);
                Ok(Expression {
                    expr: Expr::Boolean(bool),
                    location,
                })
            }
            _ => unreachable!("Unexpected expression {:?}", pair),
        }
    }
}
