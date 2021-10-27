use pest::iterators::Pair;

use crate::grammar::Rule;

use super::{Locatable, Location};

#[derive(Debug)]
pub struct Float {
    pub value: f32,
    pub location: Location,
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.value - other.value < std::f32::EPSILON && self.location.eq(&other.location)
    }
}

impl Eq for Float {}

impl<'a> From<Pair<'a, Rule>> for Float {
    fn from(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::float => {
                let value: f32 = pair.as_str().to_owned().parse().unwrap();
                let location = Location::from(&pair);

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
    use crate::grammar::{Grammar, Rule};
    use pest::Parser;

    #[test]
    fn parse_float() {
        let pair = Grammar::parse(Rule::float, "123456.789")
            .unwrap()
            .next()
            .unwrap();

        let float = Float::from(pair);
        assert_eq!(float, Float { 
            value: 123456.789,
            location: Location::new(0, 10)
        });
    }
}
