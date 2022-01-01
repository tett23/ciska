use pest::iterators::Pair;
use pest::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[grammar = "document.pest"]
pub struct DocumentParser;

pub fn parse(document: &str) -> Node {
    let pairs = DocumentParser::parse(Rule::document, document).unwrap_or_else(|e| panic!("{}", e));
    let mut nodes = pairs.map(|item| Node::from(&item)).collect::<Vec<Node>>();

    nodes.pop().unwrap().clone()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Node {
    Parent(Parent),
    Value(Value),
    Token(Token),
    #[serde(skip_serializing)]
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parent {
    pub name: String,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub name: String,
}

impl From<&Pair<'_, Rule>> for Node {
    fn from(pair: &Pair<'_, Rule>) -> Self {
        to_node(pair)
    }
}

pub fn to_node(pair: &Pair<'_, Rule>) -> Node {
    match pair.as_rule() {
        Rule::text => Node::Value(Value {
            name: "text".to_string(),
            value: pair.as_str().to_string(),
        }),
        Rule::slug => Node::Value(Value {
            name: "text".to_string(),
            value: pair.as_str().to_string(),
        }),
        Rule::word => Node::Value(Value {
            name: "text".to_string(),
            value: pair.as_str().to_string(),
        }),
        Rule::EOI => Node::Empty,
        _ => {
            let rule = pair.as_rule();
            let pairs = pair.clone().into_inner();
            let child_nodes = pairs
                .map(|item| to_node(&item))
                .filter(|item| item != &Node::Empty)
                .collect::<Vec<Node>>();

            match child_nodes.is_empty() {
                true => Node::Token(Token {
                    name: format!("{:?}", rule),
                }),
                false => Node::Parent(Parent {
                    name: format!("{:?}", rule),
                    children: child_nodes,
                }),
            }
        }
    }
}
