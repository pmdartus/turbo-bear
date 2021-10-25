use pest::iterators::Pair;

use crate::grammar::Rule;

use super::{pair_to_location, Boolean, Float, Integer, Locatable, Location};

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

impl<'a> TryFrom<Pair<'a, Rule>> for UnaryOperator {
    type Error = ();

    fn try_from(pair: Pair<'a, Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::bang => Ok(UnaryOperator::Not),
            Rule::minus => Ok(UnaryOperator::Minus),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Expression {
    expression: Expr,
    location: Location,
}

impl Locatable for Expression {
    fn location(&self) -> &Location {
        &self.location
    }
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
    Integer(Integer),
    Float(Float),
    Boolean(Boolean),
}

impl<'a> From<Pair<'a, Rule>> for Expression {
    fn from(pair: Pair<Rule>) -> Self {
        let location = pair_to_location(&pair);

        match pair.as_rule() {
            Rule::expression => {
                let inner = pair.into_inner().next().unwrap();
                Self::from(inner)
            }
            Rule::logical_or | Rule::logical_and => {
                let mut inner = pair.into_inner();
                let mut expr = Self::from(inner.next().unwrap());

                while let Some(op) = inner.next() {
                    let operator = LogicalOperator::from(op);
                    let right = Self::from(inner.next().unwrap());
                    let location = [expr.location[0], right.location[1]];

                    expr = Expression {
                        expression: Expr::Logical {
                            operator,
                            left: Box::new(expr),
                            right: Box::new(right),
                        },
                        location,
                    }
                }

                expr
            }
            Rule::equality | Rule::comparison | Rule::term | Rule::factor => {
                let mut inner = pair.into_inner();
                let mut expr = Self::from(inner.next().unwrap());

                while let Some(op) = inner.next() {
                    let operator = BinaryOperator::from(op);
                    let right = Self::from(inner.next().unwrap());
                    let location = [expr.location[0], right.location[1]];

                    expr = Expression {
                        expression: Expr::Binary {
                            operator,
                            left: Box::new(expr),
                            right: Box::new(right),
                        },
                        location,
                    }
                }

                expr
            }
            Rule::unary => {
                let mut inner = pair.into_inner();
                let next = inner.next().unwrap();

                match UnaryOperator::try_from(next.clone()) {
                    Ok(operator) => {
                        let expression = Self::from(inner.next().unwrap());

                        Expression {
                            expression: Expr::Unary {
                                operator,
                                expression: Box::new(expression),
                            },
                            location,
                        }
                    }
                    _ => Self::from(next),
                }
            }
            Rule::integer => {
                let int = Integer::from(pair);
                Expression {
                    expression: Expr::Integer(int),
                    location,
                }
            }
            Rule::float => {
                let float = Float::from(pair);
                Expression {
                    expression: Expr::Float(float),
                    location,
                }
            }
            Rule::boolean => {
                let bool = Boolean::from(pair);
                Expression {
                    expression: Expr::Boolean(bool),
                    location,
                }
            }
            _ => unreachable!("Unexpected expression {:?}", pair),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{Grammar, Rule};
    use pest::Parser;

    fn parse_expression(input: &str) -> Expression {
        let pair = Grammar::parse(Rule::expression, input)
            .unwrap()
            .next()
            .unwrap();

        Expression::from(pair)
    }

    #[test]
    fn parse_integer() {
        let expr = parse_expression("1");
        assert_eq!(
            expr,
            Expression {
                expression: Expr::Integer(Integer {
                    value: 1,
                    location: [0, 1]
                }),
                location: [0, 1]
            }
        )
    }

    #[test]
    fn parse_float() {
        let expr = parse_expression("1.2");
        assert_eq!(
            expr,
            Expression {
                expression: Expr::Float(Float {
                    value: 1.2,
                    location: [0, 3]
                }),
                location: [0, 3]
            }
        )
    }

    #[test]
    fn parse_boolean() {
        let expr = parse_expression("true");
        assert_eq!(
            expr,
            Expression {
                expression: Expr::Boolean(Boolean {
                    value: true,
                    location: [0, 4]
                }),
                location: [0, 4]
            }
        )
    }

    #[test]
    fn parse_unary() {
        let expr = parse_expression("!false");
        assert_eq!(
            expr,
            Expression {
                expression: Expr::Unary {
                    operator: UnaryOperator::Not,
                    expression: Box::new(Expression {
                        expression: Expr::Boolean(Boolean {
                            value: false,
                            location: [1, 6]
                        }),
                        location: [1, 6]
                    })
                },
                location: [0, 6]
            }
        )
    }

    #[test]
    fn parse_recursive_unary() {
        let expr = parse_expression("!!false");
        assert_eq!(
            expr,
            Expression {
                expression: Expr::Unary {
                    operator: UnaryOperator::Not,
                    expression: Box::new(Expression {
                        expression: Expr::Unary {
                            operator: UnaryOperator::Not,
                            expression: Box::new(Expression {
                                expression: Expr::Boolean(Boolean {
                                    value: false,
                                    location: [2, 7]
                                }),
                                location: [2, 7]
                            })
                        },
                        location: [1, 7 ]
                    })
                },
                location: [0, 7]
            }
        )
    }

    #[test]
    fn parse_binary() {
        let expr = parse_expression("1 + 2");
        assert_eq!(expr, Expression {
            expression: Expr::Binary {
                operator: BinaryOperator::Add,
                left: Box::new(Expression {
                    expression: Expr::Integer(Integer {
                        value: 1,
                        location: [0, 1]
                    }),
                    location: [0, 1]
                }),
                right: Box::new(Expression {
                    expression: Expr::Integer(Integer {
                        value: 2,
                        location: [4, 5]
                    }),
                    location: [4, 5]
                })
            },
            location: [0, 5]
        })
    }

    #[test]
    fn parse_binary_associativity() {
        let expr = parse_expression("2 * 3 + 4");
        assert_eq!(expr, Expression {
            expression: Expr::Binary {
                operator: BinaryOperator::Add,
                left: Box::new(Expression {
                    expression: Expr::Binary {
                        operator: BinaryOperator::Multiply,
                        left: Box::new(Expression {
                            expression: Expr::Integer(Integer {
                                value: 2,
                                location: [0, 1]
                            }),
                            location: [0, 1]
                        }),
                        right: Box::new(Expression {
                            expression: Expr::Integer(Integer {
                                value: 3,
                                location: [4, 5]
                            }),
                            location: [4, 5]
                        })
                    },
                    location: [0, 5]
                }),
                right: Box::new(Expression {
                    expression: Expr::Integer(Integer {
                        value: 4,
                        location: [8, 9]
                    }),
                    location: [8, 9]
                })
            },
            location: [0, 9]
        })
    }

    #[test]
    fn parse_grouping() {
        let expr = parse_expression("2 * (3 + 4)");
        assert_eq!(expr, Expression {
            expression: Expr::Binary {
                operator: BinaryOperator::Multiply,
                left: Box::new(Expression {
                    expression: Expr::Integer(Integer {
                        value: 2,
                        location: [0, 1]
                    }),
                    location: [0, 1]
                }),
                right: Box::new(Expression {
                    expression: Expr::Binary {
                        operator: BinaryOperator::Add,
                        left: Box::new(Expression {
                            expression: Expr::Integer(Integer {
                                value: 3,
                                location: [5, 6]
                            }),
                            location: [5, 6]
                        }),
                        right: Box::new(Expression {
                            expression: Expr::Integer(Integer {
                                value: 4,
                                location: [9, 10]
                            }),
                            location: [9, 10]
                        })
                    },
                    location: [5, 10]
                }),
            },
            location: [0, 10]
        })
    }

    #[test]
    fn parse_complex_expression() {
        parse_expression("(-1 + 2) * 3 - -4");
    }

    #[test]
    fn parse_and() {
        let expr = parse_expression("false && true");
        assert_eq!(expr, Expression {
            expression: Expr::Logical {
                operator: LogicalOperator::And,
                left: Box::new(Expression {
                    expression: Expr::Boolean(Boolean {
                        value: false,
                        location: [0, 5]
                    }),
                    location: [0, 5]
                }),
                right: Box::new(Expression {
                    expression: Expr::Boolean(Boolean {
                        value: true,
                        location: [9, 13]
                    }),
                    location: [9, 13]
                })
            },
            location: [0, 13]
        })
    }

    #[test]
    fn parse_or() {
        let expr = parse_expression("false || true");
        assert_eq!(expr, Expression {
            expression: Expr::Logical {
                operator: LogicalOperator::Or,
                left: Box::new(Expression {
                    expression: Expr::Boolean(Boolean {
                        value: false,
                        location: [0, 5]
                    }),
                    location: [0, 5]
                }),
                right: Box::new(Expression {
                    expression: Expr::Boolean(Boolean {
                        value: true,
                        location: [9, 13]
                    }),
                    location: [9, 13]
                })
            },
            location: [0, 13]
        })
    }

    #[test]
    fn parse_logical_associativity() {
        let expr = parse_expression("true && false || true && false");
        assert_eq!(expr, Expression {
            expression: Expr::Logical {
                operator: LogicalOperator::Or,
                left: Box::new(Expression {
                    expression: Expr::Logical {
                        operator: LogicalOperator::And,
                        left: Box::new(Expression {
                            expression: Expr::Boolean(Boolean {
                                value: true,
                                location: [0, 4]
                            }),
                            location: [0, 4]
                        }),
                        right: Box::new(Expression {
                            expression: Expr::Boolean(Boolean {
                                value: false,
                                location: [8, 13]
                            }),
                            location: [8, 13]
                        })
                    },
                    location: [0, 13]
                }),
                right: Box::new(Expression {
                    expression: Expr::Logical {
                        operator: LogicalOperator::And,
                        left: Box::new(Expression {
                            expression: Expr::Boolean(Boolean {
                                value: true,
                                location: [17, 21]
                            }),
                            location: [17, 21]
                        }),
                        right: Box::new(Expression {
                            expression: Expr::Boolean(Boolean {
                                value: false,
                                location: [25, 30]
                            }),
                            location: [25, 30]
                        })
                    },
                    location: [17, 30]
                })
            },
            location: [0, 30]
        })
    }
}
