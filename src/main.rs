extern crate processor;
extern crate prose_parser;

use processor::Processor;
use prose_parser::Node;

fn main() {
    println!("{}", process("").unwrap());
}

fn process(text: &str) -> Result<String, String> {
    Processor::<Node, String>::new()
        .parser(prose_parser::parse)
        .formatter(|ast| ast.to_json())
        .process(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        dbg!(process("").unwrap());
    }
}
