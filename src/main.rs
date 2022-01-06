extern crate processor;
extern crate prose_parser;

use processor::Processor;
use prose_parser::Node;

fn main() {
    println!("{}", process("text").unwrap());
}

fn process(text: &str) -> Result<String, String> {
    Processor::<Node, String>::new()
        .parser(prose_parser::parse)
        .transformer(|ast| Ok(transform_text(ast, "a")))
        .transformer(|ast| Ok(transform_text(ast, "b")))
        .formatter(|ast| ast.to_json())
        .process(text)
}

fn transform_text(node: &Node, append_text: &str) -> Node {
    match node {
        Node::Parent(parent) => {
            let mut ret = parent.clone();
            ret.children = parent
                .children
                .iter()
                .map(|item| transform_text(item, append_text))
                .collect::<Vec<_>>();

            Node::Parent(ret)
        }
        Node::Token(token) => Node::Token(token.clone()),
        Node::Empty => Node::Empty,
        Node::Value(value) => {
            let mut ret = value.clone();
            ret.value = value.value.to_string() + append_text;

            Node::Value(ret)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        dbg!(process("text").unwrap());
    }
}
