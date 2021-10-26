use pest::iterators::Pair;

use super::{Locatable, Location};
use crate::grammar::Rule;

#[derive(Debug, PartialEq, Eq)]
pub struct Boolean {
    pub value: bool,
    pub location: Location,
}

impl<'a> From<Pair<'a, Rule>> for Boolean {
    fn from(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::boolean => {
                let str = pair.as_str();
                let location = Location::from(&pair);

                match str {
                    "true" => Boolean {
                        value: true,
                        location,
                    },
                    "false" => Boolean {
                        value: false,
                        location,
                    },
                    _ => unreachable!("Unexpected boolean value {:?}", pair),
                }
            }
            _ => unreachable!("Unexpected boolean value {:?}", pair),
        }
    }
}

impl Locatable for Boolean {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{Grammar, Rule};
    use pest::Parser;

    #[test]
    fn parse_boolean_true() {
        let pair = Grammar::parse(Rule::boolean, "true")
            .unwrap()
            .next()
            .unwrap();

        let boolean = Boolean::from(pair);
        assert_eq!(
            boolean,
            Boolean {
                value: true,
                location: Location::new(0, 4)
            }
        );
    }

    #[test]
    fn parse_boolean_false() {
        let pair = Grammar::parse(Rule::boolean, "false")
            .unwrap()
            .next()
            .unwrap();

        let boolean = Boolean::from(pair);
        assert_eq!(
            boolean,
            Boolean {
                value: false,
                location: Location::new(0, 5)
            }
        );
    }
}
