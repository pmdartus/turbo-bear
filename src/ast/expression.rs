use pest::{
    iterators::Pair,
    prec_climber::{Assoc, Operator, PrecClimber},
    Parser,
};

use crate::grammar::{Grammar, Rule};

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
                    |lhs_res: Result<Expression, ParsingError>, op: Pair<Rule>, rhs_res: Result<Expression, ParsingError>| {
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
                    |lhs_res: Result<Expression, ParsingError>, op: Pair<Rule>, rhs_res: Result<Expression, ParsingError>| {
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

pub fn parse_expression(input: &str) -> Result<Expression, ParsingError> {
    let pair = Grammar::parse(Rule::whole_expression, input)
        .unwrap()
        .next()
        .unwrap();

    Expression::try_from(pair)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_integer() {
        let expr = parse_expression("1");
        assert_eq!(
            expr,
            Ok(Expression {
                expr: Expr::Integer(Integer {
                    value: 1,
                    location: Location::new(0, 1)
                }),
                location: Location::new(0, 1)
            })
        )
    }

    #[test]
    fn parse_float() {
        let expr = parse_expression("1.2");
        assert_eq!(
            expr,
            Ok(Expression {
                expr: Expr::Float(Float {
                    value: 1.2,
                    location: Location::new(0, 3)
                }),
                location: Location::new(0, 3)
            })
        )
    }

    #[test]
    fn parse_boolean() {
        let expr = parse_expression("true");
        assert_eq!(
            expr,
            Ok(Expression {
                expr: Expr::Boolean(Boolean {
                    value: true,
                    location: Location::new(0, 4)
                }),
                location: Location::new(0, 4)
            })
        )
    }

    #[test]
    fn parse_unary() {
        let expr = parse_expression("!false");
        assert_eq!(
            expr,
            Ok(Expression {
                expr: Expr::Unary {
                    operator: UnaryOperator::Not,
                    expression: Box::new(Expression {
                        expr: Expr::Boolean(Boolean {
                            value: false,
                            location: Location::new(1, 6)
                        }),
                        location: Location::new(1, 6)
                    })
                },
                location: Location::new(0, 6)
            })
        )
    }

    #[test]
    fn parse_recursive_unary() {
        let expr = parse_expression("!!false");
        assert_eq!(
            expr,
            Ok(Expression {
                expr: Expr::Unary {
                    operator: UnaryOperator::Not,
                    expression: Box::new(Expression {
                        expr: Expr::Unary {
                            operator: UnaryOperator::Not,
                            expression: Box::new(Expression {
                                expr: Expr::Boolean(Boolean {
                                    value: false,
                                    location: Location::new(2, 7)
                                }),
                                location: Location::new(2, 7)
                            })
                        },
                        location: Location::new(1, 7)
                    })
                },
                location: Location::new(0, 7)
            })
        )
    }

    #[test]
    fn parse_binary() {
        let expr = parse_expression("1 + 2");
        assert_eq!(
            expr,
            Ok(Expression {
                expr: Expr::Binary {
                    operator: BinaryOperator::Add,
                    left: Box::new(Expression {
                        expr: Expr::Integer(Integer {
                            value: 1,
                            location: Location::new(0, 1)
                        }),
                        location: Location::new(0, 1)
                    }),
                    right: Box::new(Expression {
                        expr: Expr::Integer(Integer {
                            value: 2,
                            location: Location::new(4, 5)
                        }),
                        location: Location::new(4, 5)
                    })
                },
                location: Location::new(0, 5)
            })
        )
    }

    #[test]
    fn parse_binary_associativity() {
        let expr = parse_expression("2 * 3 + 4");
        assert_eq!(
            expr,
            Ok(Expression {
                expr: Expr::Binary {
                    operator: BinaryOperator::Add,
                    left: Box::new(Expression {
                        expr: Expr::Binary {
                            operator: BinaryOperator::Multiply,
                            left: Box::new(Expression {
                                expr: Expr::Integer(Integer {
                                    value: 2,
                                    location: Location::new(0, 1)
                                }),
                                location: Location::new(0, 1)
                            }),
                            right: Box::new(Expression {
                                expr: Expr::Integer(Integer {
                                    value: 3,
                                    location: Location::new(4, 5)
                                }),
                                location: Location::new(4, 5)
                            })
                        },
                        location: Location::new(0, 5)
                    }),
                    right: Box::new(Expression {
                        expr: Expr::Integer(Integer {
                            value: 4,
                            location: Location::new(8, 9)
                        }),
                        location: Location::new(8, 9)
                    })
                },
                location: Location::new(0, 9)
            })
        )
    }

    #[test]
    fn parse_grouping() {
        let expr = parse_expression("2 * (3 + 4)");
        assert_eq!(
            expr,
            Ok(Expression {
                expr: Expr::Binary {
                    operator: BinaryOperator::Multiply,
                    left: Box::new(Expression {
                        expr: Expr::Integer(Integer {
                            value: 2,
                            location: Location::new(0, 1)
                        }),
                        location: Location::new(0, 1)
                    }),
                    right: Box::new(Expression {
                        expr: Expr::Binary {
                            operator: BinaryOperator::Add,
                            left: Box::new(Expression {
                                expr: Expr::Integer(Integer {
                                    value: 3,
                                    location: Location::new(5, 6)
                                }),
                                location: Location::new(5, 6)
                            }),
                            right: Box::new(Expression {
                                expr: Expr::Integer(Integer {
                                    value: 4,
                                    location: Location::new(9, 10)
                                }),
                                location: Location::new(9, 10)
                            })
                        },
                        location: Location::new(5, 10)
                    }),
                },
                location: Location::new(0, 10)
            })
        )
    }

    #[test]
    fn parse_complex_expression() {
        parse_expression("(-1 + 2) * 3 - -4");
    }

    #[test]
    fn parse_and() {
        let expr = parse_expression("false && true");
        assert_eq!(
            expr,
            Ok(Expression {
                expr: Expr::Logical {
                    operator: LogicalOperator::And,
                    left: Box::new(Expression {
                        expr: Expr::Boolean(Boolean {
                            value: false,
                            location: Location::new(0, 5)
                        }),
                        location: Location::new(0, 5)
                    }),
                    right: Box::new(Expression {
                        expr: Expr::Boolean(Boolean {
                            value: true,
                            location: Location::new(9, 13)
                        }),
                        location: Location::new(9, 13)
                    })
                },
                location: Location::new(0, 13)
            })
        )
    }

    #[test]
    fn parse_or() {
        let expr = parse_expression("false || true");
        assert_eq!(
            expr,
            Ok(Expression {
                expr: Expr::Logical {
                    operator: LogicalOperator::Or,
                    left: Box::new(Expression {
                        expr: Expr::Boolean(Boolean {
                            value: false,
                            location: Location::new(0, 5)
                        }),
                        location: Location::new(0, 5)
                    }),
                    right: Box::new(Expression {
                        expr: Expr::Boolean(Boolean {
                            value: true,
                            location: Location::new(9, 13)
                        }),
                        location: Location::new(9, 13)
                    })
                },
                location: Location::new(0, 13)
            })
        )
    }

    #[test]
    fn parse_logical_associativity() {
        let expr = parse_expression("true && false || true && false");
        assert_eq!(
            expr,
            Ok(Expression {
                expr: Expr::Logical {
                    operator: LogicalOperator::Or,
                    left: Box::new(Expression {
                        expr: Expr::Logical {
                            operator: LogicalOperator::And,
                            left: Box::new(Expression {
                                expr: Expr::Boolean(Boolean {
                                    value: true,
                                    location: Location::new(0, 4)
                                }),
                                location: Location::new(0, 4)
                            }),
                            right: Box::new(Expression {
                                expr: Expr::Boolean(Boolean {
                                    value: false,
                                    location: Location::new(8, 13)
                                }),
                                location: Location::new(8, 13)
                            })
                        },
                        location: Location::new(0, 13)
                    }),
                    right: Box::new(Expression {
                        expr: Expr::Logical {
                            operator: LogicalOperator::And,
                            left: Box::new(Expression {
                                expr: Expr::Boolean(Boolean {
                                    value: true,
                                    location: Location::new(17, 21)
                                }),
                                location: Location::new(17, 21)
                            }),
                            right: Box::new(Expression {
                                expr: Expr::Boolean(Boolean {
                                    value: false,
                                    location: Location::new(25, 30)
                                }),
                                location: Location::new(25, 30)
                            })
                        },
                        location: Location::new(17, 30)
                    })
                },
                location: Location::new(0, 30)
            })
        )
    }
}
