use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum ParsingError {
    ReservedKeyword { name: String },
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsingError::ReservedKeyword { name } => {
                write!(f, "Invalid identifier. '{}' is a reserved keyword.", name)
            }
        }
    }
}
