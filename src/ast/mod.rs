mod integer;
mod float;
mod boolean;
mod expression;

pub use integer::Integer;
pub use float::Float;
pub use boolean::Boolean;

use pest::iterators::Pair;
use crate::grammar::{Rule};

pub type Location = [usize; 2];

pub trait Locatable {
    fn location(&self) -> &Location;
}

pub fn pair_to_location(pair: &Pair<Rule>) -> Location {
    let span = pair.as_span();
    [span.start(), span.end()]
}