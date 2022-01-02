extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;

pub fn parse(document: &str) -> Result<parser::Node, String> {
    parser::parse(document)
}
