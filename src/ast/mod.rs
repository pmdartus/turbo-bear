mod boolean;
mod expression;
mod float;
mod integer;

pub use boolean::Boolean;
pub use float::Float;
pub use integer::Integer;

use crate::grammar::Rule;
use pest::iterators::Pair;

#[derive(Debug, PartialEq, Eq)]
pub struct Location {
    start: usize,
    end: usize,
}

impl Location {
    fn new(start: usize, end: usize) -> Self {
        Location {
            start,
            end
        }
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }
}

impl<'a> From<&Pair<'a, Rule>> for Location {
    fn from(pair: &Pair<'a, Rule>) -> Self {
        let span = pair.as_span();
        Location::new(span.start(), span.end())
    }
}

pub trait Locatable {
    fn location(&self) -> &Location;
}
