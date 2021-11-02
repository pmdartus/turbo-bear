use pest::Parser;

use crate::{grammar::{Grammar, Rule}};

use super::{Locatable, Location, ParsingErrors, Statement};

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub location: Location,
}

impl Locatable for Program {
    fn location(&self) -> &Location {
        &self.location
    }
}

pub fn parse_program(input: &str) -> Result<Program, ParsingErrors> {
    let mut pairs = Grammar::parse(Rule::program, input).unwrap();
    
    let start = 0;
    let mut end = 0;
    
    let mut errors = vec![];
    let mut statements = vec![];

    while let Some(pair) = pairs.next() {
        match pair.as_rule() {
            Rule::EOI => {
                end = pair.as_span().end()
            },
            _ => match Statement::try_from(pair) {
                Ok(stmt) => statements.push(stmt),
                Err(mut err) => errors.append(&mut err)
            }
        };
    }

    if errors.len() > 0 {
        Err(errors)
    } else {
        Ok(Program {
            statements,
            location: Location::new(start, end),
        })
    }
}