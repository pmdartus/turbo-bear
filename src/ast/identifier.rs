use pest::iterators::Pair;

use crate::grammar::Rule;

use super::{Locatable, Location, ParsingError};

#[derive(Debug, PartialEq, Eq)]
pub struct Identifier {
    pub name: String,
    pub location: Location,
}

impl Identifier {
    fn is_reserved(name: &str) -> bool {
        match name {
            "class" | "else" | "false" | "fn" | "let" | "if" | "true" => true,
            _ => false,
        }
    }
}

impl<'a> TryFrom<Pair<'a, Rule>> for Identifier {
    type Error = ParsingError;

    fn try_from(value: Pair<'a, Rule>) -> Result<Self, Self::Error> {
        match value.as_rule() {
            Rule::identifier => {
                let name = value.as_str().to_owned();
                let location = Location::from(&value);

                if Identifier::is_reserved(&name) {
                    Err(ParsingError::ReservedKeyword { name })
                } else {
                    Ok(Identifier { name, location })
                }
            }
            Rule::ty => {
                Self::try_from(value.into_inner().next().unwrap())
            }
            _ => unreachable!("Unexpected identifier value {:?}", value),
        }
    }
}

impl Locatable for Identifier {
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
    fn parse_valid_identifier() {
        let cases = ["foo", "Foo", "foo_bar", "f00_b4r"];
        for case in cases {
            let pair = Grammar::parse(Rule::identifier, case)
                .unwrap()
                .next()
                .unwrap();

            let identifier = Identifier::try_from(pair);
            assert_eq!(
                identifier,
                Ok(Identifier {
                    name: case.to_owned(),
                    location: Location::new(0, case.len())
                })
            )
        }
    }

    #[test]
    fn parse_invalid_identifier() {
        let cases = ["class", "else", "false", "fn", "let", "if", "true"];
        for case in cases {
            let pair = Grammar::parse(Rule::identifier, case)
                .unwrap()
                .next()
                .unwrap();

            let identifier = Identifier::try_from(pair);
            assert_eq!(
                identifier,
                Err(ParsingError::ReservedKeyword {
                    name: case.to_owned()
                })
            )
        }
    }
}
