use pest::iterators::Pair;

use crate::grammar::Rule;

use super::{Expression, Identifier, Locatable, Location, ParsingErrors};

#[derive(Debug, PartialEq, Eq)]
pub enum Stmt {
    VariableDeclaration {
        identifier: Identifier,
        ty: Option<Identifier>,
        init: Option<Expression>,
    },
    FunctionDeclaration {
        identifier: Identifier,
        parameters: Vec<(Identifier, Identifier)>,
        return_ty: Identifier,
        body: Vec<Statement>,
    },
    Return {
        expression: Option<Expression>,
    },
    Expression {
        expression: Expression,
    },
    Block {
        statements: Vec<Statement>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Statement {
    pub stmt: Stmt,
    pub location: Location,
}

impl Locatable for Statement {
    fn location(&self) -> &Location {
        &self.location
    }
}

impl<'a> TryFrom<Pair<'a, Rule>> for Statement {
    type Error = ParsingErrors;

    fn try_from(pair: Pair<'a, Rule>) -> Result<Self, Self::Error> {
        let location = Location::from(&pair);

        match pair.as_rule() {
            Rule::variable_declaration => {
                let mut inner = pair.into_inner();

                let identifier = Identifier::try_from(inner.next().unwrap())?;
                let mut ty: Option<Identifier> = None;
                let mut init: Option<Expression> = None;

                if let Some(next) = inner.next() {
                    match next.as_rule() {
                        Rule::ty => ty = Some(Identifier::try_from(next)?),
                        Rule::expression => init = Some(Expression::try_from(next)?),
                        _ => {
                            unreachable!("Unexpected variable declaration type or init {:?}", next)
                        }
                    }
                }

                if let Some(next) = inner.next() {
                    match next.as_rule() {
                        Rule::expression => init = Some(Expression::try_from(next)?),
                        _ => unreachable!("Unexpected variable declaration  init {:?}", next),
                    }
                }

                Ok(Statement {
                    stmt: Stmt::VariableDeclaration {
                        identifier,
                        ty,
                        init,
                    },
                    location,
                })
            }
            Rule::function_declaration => {
                let mut inner = pair.into_inner();

                let identifier = Identifier::try_from(inner.next().unwrap())?;

                let mut parameters = Vec::new();
                let mut parameter_pairs = inner.next().unwrap().into_inner();
                while let (Some(name), Some(ty)) = (parameter_pairs.next(), parameter_pairs.next())
                {
                    parameters.push((Identifier::try_from(name)?, Identifier::try_from(ty)?));
                }

                let return_ty = Identifier::try_from(inner.next().unwrap())?;

                let body_statement = Self::try_from(inner.next().unwrap())?;
                let body = match body_statement {
                    Statement {
                        stmt: Stmt::Block { statements },
                        ..
                    } => statements,
                    _ => unreachable!("Unexpected body statement {:?}", body_statement),
                };

                Ok(Statement {
                    stmt: Stmt::FunctionDeclaration {
                        identifier,
                        parameters,
                        return_ty,
                        body,
                    },
                    location,
                })
            }
            Rule::return_statement => {
                let mut inner = pair.into_inner();

                println!("{:#?}", inner);

                let expression = match inner.next() {
                    Some(expr) => Some(Expression::try_from(expr)?),
                    None => None,
                };

                Ok(Statement {
                    stmt: Stmt::Return { expression },
                    location,
                })
            }
            Rule::expression_statement => {
                let mut inner = pair.into_inner();

                let expression = Expression::try_from(inner.next().unwrap())?;
                Ok(Statement {
                    stmt: Stmt::Expression { expression },
                    location,
                })
            }
            Rule::block => {
                let mut inner = pair.into_inner();

                let mut errors = vec![];
                let mut statements = vec![];

                while let Some(pair) = inner.next() {
                    match Statement::try_from(pair) {
                        Ok(stmt) => statements.push(stmt),
                        Err(mut err) => errors.append(&mut err),
                    }
                }

                if errors.len() > 0 {
                    Err(errors)
                } else {
                    Ok(Statement {
                        stmt: Stmt::Block { statements },
                        location,
                    })
                }
            }
            _ => unreachable!("Unexpected declaration {:?}", pair),
        }
    }
}
