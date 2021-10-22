use crate::Rule;
use pest::iterators::Pair;

type Location = [usize; 2];

fn pair_to_location(pair: &Pair<Rule>) -> Location {
    let span = pair.as_span();
    [span.start(), span.end()]
}

#[derive(Debug)]
pub struct Integer {
    value: u32,
    location: Location,
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

#[derive(Debug)]
pub struct Float {
    value: f32,
    location: Location,
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

#[derive(Debug)]
pub struct Boolean {
    value: bool,
    location: Location,
}

impl<'a> From<Pair<'a, Rule>> for Boolean {
    fn from(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::boolean => {
                let inner = pair.into_inner().next().unwrap();
                let location = pair_to_location(&inner);

                match inner.as_rule() {
                    Rule::boolean_true => Boolean {
                        value: true,
                        location,
                    },
                    Rule::boolean_false => Boolean {
                        value: false,
                        location,
                    },
                    _ => unreachable!("Unexpected {:?} token inside boolean", inner),
                }
            }
            _ => unreachable!("Unexpected boolean value {:?}", pair),
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
        let pair = Grammar::parse(Rule::integer, "123456").unwrap().next().unwrap();
        let int = Integer::from(pair);

        assert_eq!(int.value, 123456);
        assert_eq!(int.location[0], 0);
        assert_eq!(int.location[1], 6);
    }

    #[test]
    fn parse_float() {
        let pair = Grammar::parse(Rule::float, "123456.789").unwrap().next().unwrap();
        let float = Float::from(pair);

        assert!((float.value - 123456.789).abs() < std::f32::EPSILON);
        assert_eq!(float.location[0], 0);
        assert_eq!(float.location[1], 10);
    }

    #[test]
    fn parse_boolean_true() {
        let pair = Grammar::parse(Rule::boolean, "true").unwrap().next().unwrap();
        let boolean = Boolean::from(pair);

        assert_eq!(boolean.value, true);
        assert_eq!(boolean.location[0], 0);
        assert_eq!(boolean.location[1], 4);
    }

    #[test]
    fn parse_boolean_false() {
        let pair = Grammar::parse(Rule::boolean, "false").unwrap().next().unwrap();
        let boolean = Boolean::from(pair);

        assert_eq!(boolean.value, false);
        assert_eq!(boolean.location[0], 0);
        assert_eq!(boolean.location[1], 5);
    }
}
