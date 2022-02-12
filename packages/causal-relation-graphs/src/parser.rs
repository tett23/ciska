use pest::iterators::Pair;
use pest::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[grammar = "crg.pest"]
pub struct DocumentParser;

pub fn parse(document: &str) -> Result<Node, String> {
    DocumentParser::parse(Rule::document, document)
        .map(|pairs| {
            let mut nodes = pairs.map(|item| Node::from(&item)).collect::<Vec<Node>>();

            nodes.pop().unwrap().clone()
        })
        .map_err(|err| err.to_string())
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

impl Node {
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|err| err.to_string())
    }

    pub fn to_yaml(&self) -> Result<String, String> {
        serde_yaml::to_string(self).map_err(|err| err.to_string())
    }
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
        // Rule::text => Node::Value(Value {
        //     name: "text".to_string(),
        //     value: pair.as_str().to_string(),
        // }),
        // Rule::slug => Node::Value(Value {
        //     name: "text".to_string(),
        //     value: pair.as_str().to_string(),
        // }),
        // Rule::ch => Node::Value(Value {
        //     name: "text".to_string(),
        //     value: pair.as_str().to_string(),
        // }),
        Rule::EOI => Node::Empty,
        _ => {
            let rule = pair.as_rule();
            let pairs = pair.clone().into_inner();
            let child_nodes = pairs
                .map(|item| to_node(&item))
                .filter(|item| item != &Node::Empty)
                .fold(vec![], |mut acc, item| {
                    #[allow(mutable_borrow_reservation_conflict)]
                    match (acc.pop(), item) {
                        (Some(Node::Value(left)), Node::Value(right)) => {
                            acc.push(Node::Value(Value {
                                name: "text".to_string(),
                                value: left.value.clone() + right.value.as_str(),
                            }));
                        }
                        (Some(left), right) => {
                            acc.push(left);
                            acc.push(right);
                        }
                        (None, right) => {
                            acc.push(right);
                        }
                    };

                    acc
                });

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
