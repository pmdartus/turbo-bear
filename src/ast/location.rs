use pest::iterators::Pair;

use crate::grammar::Rule;

#[derive(Debug, PartialEq, Eq)]
pub struct Location {
    start: usize,
    end: usize,
}

impl Location {
    pub fn new(start: usize, end: usize) -> Self {
        Location {
            start,
            end
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
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