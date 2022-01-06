extern crate processor;
extern crate prose_parser;

use processor::Processor;
use prose_parser::Node;

fn main() {
    println!("{}", parse("").unwrap().to_json().unwrap());
}

fn parse(text: &str) -> Result<Node, String> {
    Processor::<Node>::new()
        .parser(prose_parser::parse)
        .parse(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        dbg!(parse("").unwrap());
    }
}
