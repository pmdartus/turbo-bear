use std::fmt;

use pest::error::{Error, ErrorVariant, InputLocation};

use super::Rule;
use crate::ast::location::Location;

#[derive(Debug)]
pub enum ParsingErrorKind {
    Custom(String),
    ReservedKeyword(String),
    InvalidInteger(String),
    InvalidFloat(String),
    TopLevelReturn,
}

impl fmt::Display for ParsingErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParsingErrorKind::*;

        match self {
            Custom(msg) => {
                write!(f, "{}", msg)
            }
            ReservedKeyword(name) => {
                write!(f, "Invalid identifier. '{}' is a reserved keyword.", name)
            }
            InvalidInteger(value) => {
                write!(f, "Invalid integer literal. Failed to parse '{}'.", value)
            }
            InvalidFloat(value) => {
                write!(f, "Invalid float literal. Failed to parse '{}'.", value)
            }
            TopLevelReturn => {
                write!(f, "Invalid return statement. Top level code can't return.")
            }
        }
    }
}

#[derive(Debug)]
pub enum ParsingErrorLocation {
    Position(usize),
    Span((usize, usize)),
}

#[derive(Debug)]
pub struct ParsingError {
    pub kind: ParsingErrorKind,
    pub location: ParsingErrorLocation,
}

impl ParsingError {
    pub fn new(kind: ParsingErrorKind, location: Location) -> Self {
        ParsingError {
            kind,
            location: ParsingErrorLocation::Span((location.start, location.end)),
        }
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.", self.kind)
    }
}

impl From<Error<Rule>> for ParsingError {
    fn from(err: Error<Rule>) -> Self {
        let msg = match err.variant {
            ErrorVariant::ParsingError {
                mut positives,
                mut negatives,
            } => {
                let mut rules = Vec::new();
                rules.append(&mut positives);
                rules.append(&mut negatives);

                format!("Expected {}.", format_rules(rules))
            }
            ErrorVariant::CustomError { message } => message,
        };

        let location = match err.location {
            InputLocation::Pos(pos) => ParsingErrorLocation::Position(pos),
            InputLocation::Span((start, end)) => ParsingErrorLocation::Span((start, end)),
        };

        ParsingError {
            kind: ParsingErrorKind::Custom(msg),
            location,
        }
    }
}

fn format_rules(rules: Vec<Rule>) -> String {
    let formatted_rules: Vec<String> = rules
        .into_iter()
        .map(|rule| {
            match rule {
                Rule::and => "&&",
                Rule::arguments => "arguments",
                Rule::bang => "!",
                Rule::bang_equal => "!=",
                Rule::binary => "binary expression",
                Rule::binary_operator => "binary operator",
                Rule::block => "block",
                Rule::boolean => "boolean",
                Rule::call => "function call",
                Rule::EOI => "end of file",
                Rule::equal => "=",
                Rule::equal_equal => "==",
                Rule::expression => "expression",
                Rule::expression_statement => "expression statement",
                Rule::float | Rule::float_characteristic | Rule::float_mantissa => "float",
                Rule::function_declaration => "function declaration",
                Rule::greater => ">",
                Rule::greater_equal => ">=",
                Rule::identifier => "identifier",
                Rule::integer => "integer",
                Rule::less => "<",
                Rule::less_equal => "<=",
                Rule::logical => "logical expression",
                Rule::logical_operator => "logical operator",
                Rule::minus => "-",
                Rule::or => "||",
                Rule::parameters => "function parameters",
                Rule::plus => "+",
                Rule::primary => "primary expression",
                Rule::program => "program",
                Rule::return_statement => "return statement",
                Rule::slash => "/",
                Rule::star => "*",
                Rule::statement => "statement",
                Rule::top_level_decl => "top level declaration",
                Rule::ty => "type",
                Rule::unary => "unary expression",
                Rule::unary_operator => "unary operator",
                Rule::variable_declaration => "variable declaration",
                Rule::WHITESPACE => "whitespace character",
            }
            .to_owned()
        })
        .collect();

    formatted_rules.join(", ")
}
