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
    Root(ScopeValue),
    Scope(ScopeValue),
    Stmt(Stmt),
    Comment(Comment),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeValue(pub Vec<Stmt>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stmt {
    Expr(Expr),
    TypeExpr(TypeExpr),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeExpr {
    Bind(TypeSymbol),
    Assign {
        symbol: TypeSymbol,
        value: TypeValue,
    },
    Id(TypeValue),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeValue {
    Id,
    Empty,
    Int,
    TypeSymbol(TypeSymbol),
    StateMachine(StateMachine),
    Snapshot(Snapshot),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot(Vec<Context>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Context(ContextLabel, EffectType);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextLabel(String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectType {
    Id,
    Empty,
    Int,
    StateMachine,
    TypeSymbolReference(TypeSymbolName),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeSymbolReference(String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateMachine(Vec<StateLabel>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateLabel(String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeSymbol {
    name: TypeSymbolName,
    kind: TypeKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeSymbolName(String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeKind {
    StateMachine,
    Snapshot,
    Context,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expr {
    Id(Value),
    Op(Op, Box<Expr>, Box<Expr>),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Op {
    Compose,
    Apply,
    Reduce,
    Push,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddEffect(i64);
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntLiteral(i64);
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionEffect(String);
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionLiteral(String);
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Value {
    Id,
    Empty,
    AddEffect(AddEffect),
    TransitionEffect(TransitionEffect),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment(String);

impl Expr {
    pub fn eval(&self, vm: &Vm) -> (Vm, Value) {
        match self {
            Expr::Id(v) => (vm.clone(), v.clone()),
            Expr::Op(op, lhs, rhs) => {
                let (vm, lhs_value) = lhs.eval(vm);
                let (vm, rhs_value) = rhs.eval(&vm);

                (vm.clone(), op.apply(lhs_value, rhs_value))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scope {
    type_symbols: Vec<(TypeSymbol, TypeValue)>,
    return_value: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vm {
    stack: Vec<Scope>,
}

impl Vm {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push_stack(&mut self) {
        self.stack.push(Scope::new())
    }

    pub fn pop_stack(&mut self) -> Option<Scope> {
        self.stack.pop()
    }

    pub fn current_scope(&mut self) -> &mut Scope {
        self.stack.last_mut().unwrap()
    }

    // pub fn current_return(&mut self) -> Value {
    //     self.current_scope().return_value()
    // }
}

impl Scope {
    pub fn new() -> Self {
        Self {
            type_symbols: Vec::new(),
            return_value: Value::Empty,
        }
    }

    pub fn push_type_symbol(&mut self, symbol: &TypeSymbol) {
        self.type_symbols.push((symbol.clone(), TypeValue::Empty));
    }

    pub fn assign_type_symbol(&mut self, symbol: &TypeSymbol, value: &TypeValue) {
        self.type_symbols.push((symbol.clone(), value.clone()));
    }

    pub fn push_return_value(&mut self, value: &Value) {
        self.return_value = value.clone()
    }

    pub fn return_value(&self) -> Value {
        self.return_value.clone()
    }
}

impl TypeExpr {
    pub fn eval(&self, vm: &Vm) -> (Vm, TypeValue) {
        match self {
            Self::Bind(symbol) => {
                let mut vm = vm.clone();
                vm.current_scope().push_type_symbol(&symbol);

                (vm.clone(), TypeValue::TypeSymbol(symbol.clone()))
            }
            Self::Assign { symbol, value } => {
                let mut vm = vm.clone();
                vm.current_scope().assign_type_symbol(symbol, value);

                (vm.clone(), value.clone())
            }
            Self::Id(value) => (vm.clone(), value.clone()),
        }
    }
}

impl Stmt {
    pub fn eval(&self) -> Value {
        Value::Empty
    }
}

impl ScopeValue {
    pub fn eval(&self, vm: &Vm) -> Value {
        let mut vm = vm.clone();
        vm.push_stack();

        let mut vm = self.0.iter().fold(vm, |vm, stmt| {
            let vm = match stmt {
                Stmt::Expr(expr) => {
                    let (mut vm, value) = expr.eval(&vm);
                    vm.current_scope().push_return_value(&value);

                    vm
                }
                Stmt::TypeExpr(expr) => {
                    let (vm, _) = expr.eval(&vm);

                    vm
                }
            };

            vm
        });

        let scope = vm.pop_stack();
        dbg!(&scope);

        scope.unwrap().return_value()
    }
}

impl Op {
    pub fn apply(&self, lhs: Value, rhs: Value) -> Value {
        match (self, lhs, rhs) {
            (Op::Compose, lhs, rhs) => eval_compose(lhs, rhs),
            (Op::Apply, _lhs, _rhs) => unimplemented!(),
            (Op::Reduce, _lhs, _rhs) => unimplemented!(),
            (Op::Push, _lhs, _rhs) => unimplemented!(),
        }
    }
}

fn eval_compose(lhs: Value, rhs: Value) -> Value {
    match (lhs, rhs) {
        (Value::Id, Value::Id) => Value::Id,
        (Value::AddEffect(lhs), Value::Id) => Value::AddEffect(lhs),
        (Value::Id, Value::AddEffect(rls)) => Value::AddEffect(rls),
        (Value::AddEffect(lhs), Value::AddEffect(rhs)) => {
            Value::AddEffect(AddEffect(lhs.0 + rhs.0))
        }
        (Value::TransitionEffect(lhs), Value::Id) => Value::TransitionEffect(lhs),
        (Value::Id, Value::TransitionEffect(rls)) => Value::TransitionEffect(rls),
        (Value::TransitionEffect(_lhs), Value::TransitionEffect(rhs)) => {
            // NOTE: 遷移可能かのチェックをする
            // Effect単体でTransitionの合成できないのではという疑惑ある
            Value::TransitionEffect(TransitionEffect(rhs.0))
        }
        _ => Value::Empty,
    }
}

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
    let pair = pair.clone().into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::expr => Stmt::Expr(parse_expr(&pair)),
        Rule::typeStmt => Stmt::TypeExpr(parse_type_expr(&pair)),
        _ => panic!(),
    }
}

fn parse_type_expr(pair: &Pair<'_, Rule>) -> TypeExpr {
    let pair = pair.clone().into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::bindTypeExpr => TypeExpr::Bind(parse_type_bind(&pair)),
        Rule::assignTypeExpr => parse_type_assign(&pair),
        Rule::typeExpr => {
            unimplemented!()
        }
        _ => panic!(),
    }
}

fn parse_type_bind(pair: &Pair<'_, Rule>) -> TypeSymbol {
    let a = pair.clone().into_inner();
    let size = pair
        .clone()
        .into_inner()
        .map(|_item| true)
        .collect::<Vec<_>>()
        .len();
    match size {
        2 => {
            let a = a.map(|item| item).collect::<Vec<_>>();
            let mut a = a.iter();

            let lhs = a.next().unwrap();
            let rhs = a.next().unwrap();

            TypeSymbol {
                name: parse_type_symbol(lhs),
                kind: parse_type_keyword(rhs),
            }
        }
        _ => panic!(),
    }
}

fn parse_type_assign(pair: &Pair<'_, Rule>) -> TypeExpr {
    let a = pair.clone().into_inner();
    let size = pair
        .clone()
        .into_inner()
        .map(|_item| true)
        .collect::<Vec<_>>()
        .len();
    match size {
        2 => {
            let a = a.map(|item| item).collect::<Vec<_>>();
            let mut a = a.iter();

            let lhs = a.next().unwrap();
            let rhs = a.next().unwrap();

            TypeExpr::Assign {
                symbol: parse_type_bind(lhs),
                value: parse_type_value(rhs),
            }
        }
        _ => panic!(),
    }
}

fn parse_type_value(pair: &Pair<'_, Rule>) -> TypeValue {
    match pair.as_rule() {
        Rule::typeExpr => {
            let pair = pair.clone().into_inner().next().unwrap();

            parse_type_literal(&pair)
        }
        _ => unimplemented!(),
    }
}

fn parse_type_literal(pair: &Pair<'_, Rule>) -> TypeValue {
    match pair.as_rule() {
        Rule::typeLiteral => {
            let pair = pair.clone().into_inner().next().unwrap();
            match pair.as_rule() {
                Rule::stateMachineTypeExpr => {
                    TypeValue::StateMachine(parse_state_machine_type_expr(&pair))
                }
                Rule::snapshotTypeExpr => TypeValue::Snapshot(parse_snapshot_type_expr(&pair)),
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

fn parse_snapshot_type_expr(pair: &Pair<'_, Rule>) -> Snapshot {
    match pair.as_rule() {
        Rule::snapshotTypeExpr => {
            let pair = pair.clone().into_inner().next().unwrap();

            parse_snapshot_type_literal(&pair)
        }
        _ => unimplemented!(),
    }
}

fn parse_snapshot_type_literal(pair: &Pair<'_, Rule>) -> Snapshot {
    // TODO: & によるStateMachineの合成がある。あと、途中に式が挟まることがある
    match pair.as_rule() {
        Rule::snapshotTypeLiteral => {
            let contexts = pair
                .clone()
                .into_inner()
                .map(|item| parse_snapshot_type_item_literal(&item))
                .collect::<Vec<_>>();

            Snapshot(contexts)
        }
        _ => unimplemented!(),
    }
}

fn parse_context_type(pair: &Pair<'_, Rule>) -> Context {
    match pair.as_rule() {
        Rule::snapshotTypeItemLiteral => {
            let pair = pair.clone().into_inner().next().unwrap();
            parse_snapshot_type_item_literal(&pair)
        }
        _ => unimplemented!(),
    }
}

fn parse_snapshot_type_item_literal(pair: &Pair<'_, Rule>) -> Context {
    let a = pair.clone().into_inner();
    let size = pair
        .clone()
        .into_inner()
        .map(|_item| true)
        .collect::<Vec<_>>()
        .len();

    match size {
        1 => unimplemented!(),
        2 => {
            let a = a.map(|item| item).collect::<Vec<_>>();
            let mut a = a.iter();

            let lhs = a.next().unwrap();
            let rhs = a.next().unwrap();

            Context(parse_context_label(lhs), parse_effect_type_expr(rhs))
        }
        _ => panic!(),
    }
}

fn parse_context_label(pair: &Pair<'_, Rule>) -> ContextLabel {
    ContextLabel(parse_var_symbol(pair))
}

fn parse_var_symbol(pair: &Pair<'_, Rule>) -> String {
    match pair.as_rule() {
        Rule::varSymbol => pair.as_span().as_str().to_string(),
        _ => panic!(),
    }
}

fn parse_effect_type_expr(pair: &Pair<'_, Rule>) -> EffectType {
    match pair.as_rule() {
        Rule::effectTypeExpr => {
            let pair = pair.clone().into_inner().next().unwrap();

            parse_effect_type_literal(&pair)
        }
        _ => unimplemented!(),
    }
}

fn parse_effect_type_literal(pair: &Pair<'_, Rule>) -> EffectType {
    match pair.as_rule() {
        Rule::effectTypeLiteral => {
            let pair = pair.clone().into_inner().next().unwrap();

            match pair.as_rule() {
                Rule::intContextTypeLiteral => parse_int_context_type_literal(&pair),
                Rule::typeSymbol => EffectType::TypeSymbolReference(parse_type_symbol(&pair)),
                _ => unimplemented!(),
            }
        }
        _ => panic!(),
    }
}

fn parse_int_context_type_literal(pair: &Pair<'_, Rule>) -> EffectType {
    match pair.as_rule() {
        Rule::intContextTypeLiteral => EffectType::Int,
        _ => panic!(),
    }
}

fn parse_state_machine_type_expr(pair: &Pair<'_, Rule>) -> StateMachine {
    match pair.as_rule() {
        Rule::stateMachineTypeExpr => {
            let pair = pair.clone().into_inner().next().unwrap();

            parse_state_machine_type_literal(&pair)
        }
        _ => unimplemented!(),
    }
}

fn parse_state_machine_type_literal(pair: &Pair<'_, Rule>) -> StateMachine {
    // TODO: & によるStateMachineの合成がある。あと、途中に式が挟まることがある
    match pair.as_rule() {
        Rule::stateMachineTypeLiteral => {
            let states = pair
                .clone()
                .into_inner()
                .map(|item| parse_state_literal(&item))
                .collect::<Vec<_>>();

            StateMachine(states)
        }
        _ => unimplemented!(),
    }
}

fn parse_state_literal(pair: &Pair<'_, Rule>) -> StateLabel {
    match pair.as_rule() {
        Rule::stateLiteral => {
            let pair = pair.clone().into_inner().next().unwrap();

            StateLabel(pair.as_span().as_str().to_string())
        }
        _ => unimplemented!(),
    }
}

fn parse_type_symbol(pair: &Pair<'_, Rule>) -> TypeSymbolName {
    TypeSymbolName(pair.as_span().as_str().to_string())
}

fn parse_type_keyword(pair: &Pair<'_, Rule>) -> TypeKind {
    let pair = pair.clone().into_inner().next().unwrap();

    parse_type_kind(&pair)
}

fn parse_type_kind(pair: &Pair<'_, Rule>) -> TypeKind {
    match pair.as_rule() {
        Rule::stateMachineKeyword => TypeKind::StateMachine,
        Rule::snapshotKeyword => TypeKind::Snapshot,
        _ => panic!(),
    }
}

fn parse_op(pair: &Pair<'_, Rule>) -> Op {
    let pair = pair.clone().into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::composeOp => return Op::Compose,
        Rule::applyOp => return Op::Apply,
        Rule::reduceOp => return Op::Reduce,
        Rule::pushOp => return Op::Push,
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
    let a = pair.clone().into_inner().next().unwrap();

    Expr::Id(match a.as_rule() {
        Rule::addLiteral => Value::AddEffect(parse_add_effect(&a)),
        Rule::transitionLiteral => Value::TransitionEffect(parse_transition_effect(&a)),
        Rule::idLiteral => Value::Id,
        Rule::emptyLiteral => Value::Empty,
        _ => panic!(),
    })
}

fn parse_add_effect(pair: &Pair<'_, Rule>) -> AddEffect {
    let a = pair.clone().into_inner().next().unwrap();
    let a = parse_int_literal(&a);

    AddEffect(a.0)
}

fn parse_int_literal(pair: &Pair<'_, Rule>) -> IntLiteral {
    IntLiteral(pair.as_span().as_str().parse::<i64>().unwrap())
}

fn parse_transition_effect(pair: &Pair<'_, Rule>) -> TransitionEffect {
    let a = pair.clone().into_inner().next().unwrap();
    let a = parse_transition_literal(&a);

    TransitionEffect(a.0)
}

fn parse_transition_literal(pair: &Pair<'_, Rule>) -> TransitionLiteral {
    TransitionLiteral(pair.as_span().as_str().to_string())
}

fn parse_comment(pair: &Pair<'_, Rule>) -> Comment {
    Comment(pair.as_span().as_str().to_string())
}

fn parse_stmt_or_comment(pair: &Pair<'_, Rule>) -> Option<Stmt> {
    match pair.as_rule() {
        Rule::commentLine => None,
        Rule::stmt => Some(parse_stmt(pair)),
        _ => {
            unimplemented!()
        }
    }
}

fn parse_scope(pair: &Pair<'_, Rule>) -> ScopeValue {
    match pair.as_rule() {
        Rule::rootScope => {
            let pairs = pair.clone().into_inner();
            let a = pairs
                .map(|item| parse_stmt_or_comment(&item))
                .filter(|item| item.is_some())
                .map(|item| item.unwrap())
                .collect::<Vec<_>>();
            ScopeValue(a)
        }
        _ => {
            unimplemented!()
        }
    }
    // Comment(pair.as_span().as_str().to_string())
}

pub fn to_node(pair: &Pair<'_, Rule>) -> Node {
    match pair.as_rule() {
        Rule::document => {
            let pairs = pair.clone().into_inner();
            let pair = pairs.clone().next().unwrap();
            Node::Scope(parse_scope(&pair))
        }
        // Rule::rootScope => {
        //     let pairs = pair.clone().into_inner();
        //     let child_nodes = pairs.map(|item| parse_stmt(&item)).collect::<Vec<_>>();

        //     Node::Scope(ScopeValue(child_nodes))
        // }
        // Rule::stmt => Node::Stmt(parse_stmt(pair)),
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
