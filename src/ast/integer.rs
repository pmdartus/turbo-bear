use pest::iterators::Pair;

use crate::grammar::{Rule};
use super::{Locatable, Location, pair_to_location};

#[derive(Debug, PartialEq, Eq)]
pub struct Integer {
    pub value: u32,
    pub location: Location,
}

impl<'a> From<Pair<'a, Rule>> for Integer {
    fn from(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::integer => {
                let value: u32 = pair.as_str().to_owned().parse().unwrap();
                let location = pair_to_location(&pair);

                Integer { value, location }
            }
            _ => unreachable!("Unexpected integer value {:?}", pair),
        }
    }
}

impl Locatable for Integer {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{grammar::{Grammar, Rule}};
    use pest::Parser;

    #[test]
    fn parse_integer() {
        let pair = Grammar::parse(Rule::integer, "123456").unwrap().next().unwrap();
        
        let int = Integer::from(pair);
        assert_eq!(int, Integer {
            value: 123456,
            location: [0, 6]
        });
    }
}