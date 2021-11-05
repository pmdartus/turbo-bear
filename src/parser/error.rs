use std::fmt;

use pest::error::{Error, ErrorVariant, InputLocation};

use crate::ast::location::{Location};

use super::Rule;

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
            location: ParsingErrorLocation::Span((location.start, location.end))
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
                positives,
                negatives,
            } => { 
                let mut rules = Vec::new();
                rules.append(&mut positives.clone());
                rules.append(&mut negatives.clone());

                format!("Expected {}.", format_rules(rules))
            },
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
                Rule::EOI => "end of file",
                Rule::program => "program",
                Rule::statement => "statement",
                Rule::block => "block",
                Rule::variable_declaration => "variable declaration",
                Rule::function_declaration => "function declaration",
                Rule::function_params => "function parameters",
                Rule::expression_statement => "expression statement",
                Rule::return_statement => "return statement",
                Rule::plus => "+",
                Rule::minus => "-",
                Rule::star => "*",
                Rule::slash => "/",
                Rule::equal => "=",
                Rule::equal_equal => "==",
                Rule::bang => "!",
                Rule::bang_equal => "!=",
                Rule::greater => ">",
                Rule::greater_equal => ">=",
                Rule::less => "<",
                Rule::less_equal => "<=",
                Rule::and => "&&",
                Rule::or => "||",
                Rule::unary_operator => "unary operator",
                Rule::logical_operator => "logical operator",
                Rule::binary_operator => "binary operator",
                Rule::expression => "expression",
                Rule::logical => "logical expression",
                Rule::binary => "binary expression",
                Rule::unary => "unary expression",
                Rule::primary => "primary expression",
                Rule::identifier => "identifier",
                Rule::ty => "type",
                Rule::float | Rule::float_characteristic | Rule::float_mantissa => "float",
                Rule::integer => "integer",
                Rule::boolean => "boolean",
                Rule::WHITESPACE => "whitespace character",
            }
            .to_owned()
        })
        .collect();

    formatted_rules.join(", ")
}
