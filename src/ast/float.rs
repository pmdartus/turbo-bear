use pest::iterators::Pair;

use crate::grammar::{Rule};
use super::{Locatable, Location, pair_to_location};

#[derive(Debug)]
pub struct Float {
    pub value: f32,
    pub location: Location,
}

impl<'a> From<Pair<'a, Rule>> for Float {
    fn from(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::float => {
                let value: f32 = pair.as_str().to_owned().parse().unwrap();
                let location = pair_to_location(&pair);

                Float { value, location }
            }
            _ => unreachable!("Unexpected float value {:?}", pair),
        }
    }
}

impl Locatable for Float {
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
    fn parse_float() {
        let pair = Grammar::parse(Rule::float, "123456.789").unwrap().next().unwrap();
        let float = Float::from(pair);

        assert!((float.value - 123456.789).abs() < std::f32::EPSILON);
        assert_eq!(float.location[0], 0);
        assert_eq!(float.location[1], 10);
    }
}
