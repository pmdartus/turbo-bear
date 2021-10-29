use pest::iterators::Pair;

use crate::grammar::Rule;

use super::{Expression, Identifier, Locatable, Location, ParsingError};

#[derive(Debug, PartialEq, Eq)]
pub enum Stmt {
    VariableDeclaration {
        identifier: Identifier,
        init: Option<Expression>,
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
    type Error = ParsingError;

    fn try_from(pair: Pair<'a, Rule>) -> Result<Self, Self::Error> {
        let location = Location::from(&pair);

        match pair.as_rule() {
            Rule::variable_declaration => {
                let mut inner = pair.into_inner();

                let identifier = Identifier::try_from(inner.next().unwrap())?;
                let init = match inner.next() {
                    Some(value) => Some(Expression::try_from(value)?),
                    None => None,
                };

                Ok(Statement {
                    stmt: Stmt::VariableDeclaration { identifier, init },
                    location,
                })
            }
            _ => unreachable!("Unexpected declaration {:?}", pair),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{Expr, Integer},
        grammar::{Grammar, Rule},
    };
    use pest::Parser;

    #[test]
    fn parse_variable_declaration() {
        let pair = Grammar::parse(Rule::declaration, "let foo = 1;")
            .unwrap()
            .next()
            .unwrap();

        let declaration = Statement::try_from(pair);
        assert_eq!(
            declaration,
            Ok(Statement {
                stmt: Stmt::VariableDeclaration {
                    identifier: Identifier {
                        name: "foo".to_owned(),
                        location: Location::new(4, 7)
                    },
                    init: Some(Expression {
                        expr: Expr::Integer(Integer {
                            value: 1,
                            location: Location::new(10, 11)
                        }),
                        location: Location::new(10, 11)
                    })
                },
                location: Location::new(0, 12)
            })
        )
    }
}
