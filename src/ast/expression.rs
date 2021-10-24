use pest::iterators::{Pair};

use crate::grammar::Rule;

use super::{Boolean, Float, Integer};

#[derive(Debug)]
pub enum LogicalOperator {
    And,
    Or,
}

impl<'a> From<Pair<'a, Rule>> for LogicalOperator {
    fn from(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::and => LogicalOperator::And,
            Rule::or => LogicalOperator::Or,
            _ => unreachable!("Invalid logical operator {:?}", pair)
        }
    }
}

#[derive(Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Star,
    Slash,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl<'a> From<Pair<'a, Rule>> for BinaryOperator {
    fn from(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::plus => BinaryOperator::Plus,
            Rule::minus => BinaryOperator::Minus,
            Rule::star => BinaryOperator::Star,
            Rule::slash => BinaryOperator::Slash,
            Rule::greater => BinaryOperator::Greater,
            Rule::greater_equal => BinaryOperator::GreaterEqual,
            Rule::less => BinaryOperator::Less,
            Rule::less_equal => BinaryOperator::LessEqual,
            _ => unreachable!("Invalid binary operator {:?}", pair)
        }
    }
}

#[derive(Debug)]
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
            _ => Err(())
        }
    }
}

#[derive(Debug)]
pub enum Expression {
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
        match pair.as_rule() {
            Rule::expression => {
                let inner = pair.into_inner().next().unwrap();
                Self::from(inner)
            },
            Rule::logical_or | Rule::logical_and => {
                let mut inner = pair.into_inner();
                let mut expr = Self::from(inner.next().unwrap());

                while let Some(op) = inner.next() {
                    let operator = LogicalOperator::from(op);
                    let right = Self::from(inner.next().unwrap());

                    expr = Expression::Logical {
                        operator,
                        left: Box::new(expr),
                        right: Box::new(right)
                    }
                }

                expr
            },
            Rule::equality | Rule::comparison | Rule::term | Rule::factor => {
                let mut inner = pair.into_inner();
                let mut expr = Self::from(inner.next().unwrap());

                while let Some(op) = inner.next() {
                    let operator = BinaryOperator::from(op);
                    let right = Self::from(inner.next().unwrap());

                    expr = Expression::Binary {
                        operator,
                        left: Box::new(expr),
                        right: Box::new(right)
                    }
                }

                expr
            },
            Rule::unary => {
                let mut inner = pair.into_inner();
                let next = inner.next().unwrap();

                match UnaryOperator::try_from(next.clone()) {
                    Ok(operator) => {
                        let expression = Self::from(inner.next().unwrap());

                        Expression::Unary {
                            operator,
                            expression: Box::new(expression)
                        }
                    }
                    _ => Self::from(next)
                }
            },
            Rule::integer => Expression::Integer(Integer::from(pair)),
            Rule::float => Expression::Float(Float::from(pair)),
            Rule::boolean => Expression::Boolean(Boolean::from(pair)),
            _ => unreachable!("Unexpected expression {:?}", pair)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{grammar::{Grammar, Rule}};
    use pest::Parser;

    #[test]
    fn parse_integer() {
        let pairs = Grammar::parse(Rule::expression, "(1 + 3) * 2").unwrap();
        println!("{:#?}", pairs);

        let exp = Expression::from(pairs.clone().next().unwrap());
        println!("{:#?}", exp);
    }
}