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
    Root(Vec<Node>),
    Expr(),
    Symbol(),
    Literal(),
    IntLiteral(i64),
    AddEffect(i64),
    Stmt(Stmt),
    Keyword(),
    Comment(Comment),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stmt {
    Expr(Expr),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expr {
    Id(Value),
    Op(Op, Box<Expr>, Box<Expr>),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Op {
    Compose,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddEffect(i64);
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntLiteral(i64);
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Value {
    Id,
    Empty,
    AddEffect(AddEffect),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment(String);

impl Node {
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|err| err.to_string())
    }

    pub fn to_yaml(&self) -> Result<String, String> {
        serde_yaml::to_string(self).map_err(|err| err.to_string())
    }
}

impl From<&Pair<'_, Rule>> for Node {
    fn from(pair: &Pair<'_, Rule>) -> Self {
        to_node(pair)
    }
}

fn parse_stmt(pair: &Pair<'_, Rule>) -> Stmt {
    let a = pair
        .clone()
        .into_inner()
        .map(|item| parse_expr(&item))
        .collect::<Vec<_>>();
    let a = pair.clone().into_inner().next().unwrap();
    let a = parse_expr(&a);

    Stmt::Expr(a)
}

fn parse_op(pair: &Pair<'_, Rule>) -> Op {
    // Compose以外実装されていない
    match pair.as_rule() {
        Rule::op => Op::Compose,
        _ => unimplemented!(),
    }
}

fn parse_expr(pair: &Pair<'_, Rule>) -> Expr {
    let a = pair.clone().into_inner();
    let size = pair
        .clone()
        .into_inner()
        .map(|_item| true)
        .collect::<Vec<_>>()
        .len();
    match size {
        1 => {
            let v = a.clone().next().unwrap();
            match v.as_rule() {
                Rule::term => parse_term(&v),
                Rule::expr => parse_expr(&v),
                _ => panic!(),
            }
        }
        3 => {
            let a = a.map(|item| item).collect::<Vec<_>>();
            let mut a = a.iter();

            let v = a.next().unwrap();
            let lhs = match v.as_rule() {
                Rule::term => parse_term(&v),
                Rule::expr => parse_expr(&v),
                _ => panic!(),
            };

            let v = a.next().unwrap();
            let op = parse_op(&v);

            let v = a.next().unwrap();
            let rhs = match v.as_rule() {
                Rule::term => parse_term(&v),
                Rule::expr => parse_expr(&v),
                _ => panic!(),
            };

            Expr::Op(op, Box::new(lhs), Box::new(rhs))
        }
        _ => panic!(),
    }
}

fn parse_term(pair: &Pair<'_, Rule>) -> Expr {
    Expr::Id(Value::AddEffect(parse_add_effect(pair)))
}

fn parse_add_effect(pair: &Pair<'_, Rule>) -> AddEffect {
    let a = pair.clone().into_inner().next().unwrap();
    let a = parse_int_literal(&a);

    AddEffect(a.0)
}

fn parse_int_literal(pair: &Pair<'_, Rule>) -> IntLiteral {
    IntLiteral(pair.as_span().as_str().parse::<i64>().unwrap())
}

fn parse_comment(pair: &Pair<'_, Rule>) -> Comment {
    Comment(pair.as_span().as_str().to_string())
}

pub fn to_node(pair: &Pair<'_, Rule>) -> Node {
    match pair.as_rule() {
        Rule::document => {
            let pairs = pair.clone().into_inner();
            let child_nodes = pairs.map(|item| to_node(&item)).collect::<Vec<_>>();

            Node::Root(child_nodes)
        }
        Rule::stmt => Node::Stmt(parse_stmt(pair)),
        // Rule::expr => {
        //     let pairs = pair.clone().into_inner();
        //     let child_nodes = pairs.map(|item| to_node(&item)).collect::<Vec<_>>();

        //     Node::Expr(child_nodes)
        // }
        // Rule::addLiteral => {
        //     let pairs = pair.clone().into_inner();
        //     let child_nodes = pairs.map(|item| to_node(&item)).collect::<Vec<_>>();
        //     let a = child_nodes.first().unwrap();
        //     if let Node::IntLiteral(b) = a {
        //         return Node::AddEffect(b.clone());
        //     }

        //     unimplemented!()
        // }
        // Rule::intLiteral => Node::IntLiteral(pair.as_span().as_str().parse::<i64>().unwrap()),
        Rule::comment => Node::Comment(parse_comment(pair)),
        Rule::EOI => Node::Comment(Comment("".to_string())),
        _ => {
            unimplemented!();
        }
    }

    // match pair.as_rule() {
    //     // Rule::text => Node::Value(Value {
    //     //     name: "text".to_string(),
    //     //     value: pair.as_str().to_string(),
    //     // }),
    //     // Rule::slug => Node::Value(Value {
    //     //     name: "text".to_string(),
    //     //     value: pair.as_str().to_string(),
    //     // }),
    //     // Rule::ch => Node::Value(Value {
    //     //     name: "text".to_string(),
    //     //     value: pair.as_str().to_string(),
    //     // }),
    //     Rule::EOI => Node::Empty,
    //     _ => {
    //         let rule = pair.as_rule();
    //         let pairs = pair.clone().into_inner();
    //         let child_nodes = pairs
    //             .map(|item| to_node(&item))
    //             .filter(|item| item != &Node::Empty)
    //             .fold(vec![], |mut acc, item| {
    //                 #[allow(mutable_borrow_reservation_conflict)]
    //                 match (acc.pop(), item) {
    //                     (Some(Node::Value(left)), Node::Value(right)) => {
    //                         acc.push(Node::Value(Value {
    //                             name: "text".to_string(),
    //                             value: left.value.clone() + right.value.as_str(),
    //                         }));
    //                     }
    //                     (Some(left), right) => {
    //                         acc.push(left);
    //                         acc.push(right);
    //                     }
    //                     (None, right) => {
    //                         acc.push(right);
    //                     }
    //                 };

    //                 acc
    //             });

    //         match child_nodes.is_empty() {
    //             true => Node::Token(Token {
    //                 name: format!("{:?}", rule),
    //             }),
    //             false => Node::Parent(Parent {
    //                 name: format!("{:?}", rule),
    //                 children: child_nodes,
    //             }),
    //         }
    //     }
    // }
}
