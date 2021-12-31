extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "document.pest"]
struct DocumentParser;

#[derive(Debug)]
enum Node {
    Parent(Parent),
    Value(Value),
}

#[derive(Debug)]
struct Parent {
    pub name: String,
    pub children: Vec<Node>,
}

#[derive(Debug)]
struct Value {
    pub name: String,
    pub value: String,
}

fn to_node(pair: Pair<'_, Rule>) -> Node {
    match pair.as_rule() {
        Rule::text => Node::Value(Value {
            name: "text".to_string(),
            value: pair.as_str().to_string(),
        }),
        _ => Node::Parent(Parent {
            name: pair.as_span().as_str().to_string(),
            children: pair.into_inner().map(to_node).collect::<Vec<Node>>(),
        }),
    }
}

pub fn parse(document: &str) {
    let pairs = DocumentParser::parse(Rule::document, document).unwrap_or_else(|e| panic!("{}", e));

    let nodes = pairs.map(to_node).collect::<Vec<Node>>();
    dbg!(&nodes);
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn it_works() {
        parse("");
        parse("\n");
        parse("%% foo\n# hoge");
        parse("# foo");
        parse("## foo");
        parse("# #foo");
        parse("# foo\n\nbar\n");
        parse("1\n2\n3");
    }
}
