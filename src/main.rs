#[macro_use]
extern crate pest_derive;
extern crate pest;

mod ast;
mod grammar;

use pest::Parser;

use grammar::{Grammar, Rule};

fn main() {
    let res = Grammar::parse(Rule::boolean, "false");

    for pair in res.unwrap() {
        let node: ast::Boolean = pair.into();
        println!("{:?}", node);
    }
}
