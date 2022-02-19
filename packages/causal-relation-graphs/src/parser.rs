use pest::iterators::Pair;
use pest::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeValue(pub Vec<ScopeInnerValue>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum ScopeInnerValue {
    Comment(Comment),
    Stmt(Stmt),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stmt {
    Expr(Expr),
    TypeStmt(TypeExpr),
    LetStmt(LetExpr),
    ScopeStmt(ScopeValue),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LetExpr {
    Bind(ValueSymbol),
    Assign { symbol: ValueSymbol, expr: Expr },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueSymbol {
    name: ValueSymbolName,
    kind: BindKeywords,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BindKeywords {
    Slice,
    Effect,
    ContextEffect,
    Context,
    Snapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueSymbolName(String);

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
    Snapshot(SnapshotType),
    Context(ContextType),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotType(Vec<ContextType>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotValue(Vec<SnapshotValueItem>);

impl SnapshotValue {
    fn find_by_label(&self, label: &ContextLabel) -> Option<&SnapshotValueItem> {
        self.0
            .iter()
            .find(|SnapshotValueItem(snapshot_label, _)| snapshot_label == label)
    }

    fn insert(&self, label: &ContextLabel, value: &SnapshotValueItemValue) -> SnapshotValue {
        let mut ret = self.clone();

        let idx = self
            .0
            .iter()
            .position(|SnapshotValueItem(label, _)| label == label);
        match idx {
            Some(idx) => {
                ret.0[idx] = SnapshotValueItem(label.clone(), value.clone());
            }
            None => ret.0.push(SnapshotValueItem(label.clone(), value.clone())),
        }

        ret
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotValueItem(ContextLabel, SnapshotValueItemValue);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnapshotValueItemValue {
    Id,
    Empty,
    Int(IntLiteral),
    StateLabel(StateLabel),
}

impl SnapshotValueItemValue {
    fn apply(&self, effect: &Effect) -> SnapshotValueItemValue {
        match self {
            SnapshotValueItemValue::Id => SnapshotValueItemValue::Empty,
            SnapshotValueItemValue::Empty => SnapshotValueItemValue::Empty,
            SnapshotValueItemValue::Int(v) => match effect {
                Effect::Id => SnapshotValueItemValue::Int(IntLiteral(v.0)),
                Effect::AddEffect(vv) => SnapshotValueItemValue::Int(IntLiteral(v.0 + vv.0)),
                _ => SnapshotValueItemValue::Empty,
            },
            SnapshotValueItemValue::StateLabel(v) => match effect {
                Effect::Id => SnapshotValueItemValue::StateLabel(v.clone()),
                Effect::TransitionEffect(_vv) => {
                    unimplemented!()
                }
                _ => SnapshotValueItemValue::Empty,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextType(ContextLabel, EffectType);

// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub struct ContextValue(ContextReference);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct ContextLabel(String);

// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub struct ContextReference(String);

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
pub enum StateLabel {
    Id,
    Empty,
    Label(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeSymbol {
    name: TypeSymbolName,
    kind: TypeKind,
}

trait Assignable<T> {
    fn assignable(&self, value: &T) -> bool;
}

impl Assignable<TypeValue> for TypeSymbol {
    fn assignable(&self, _value: &TypeValue) -> bool {
        // TODO: 実装

        true
    }
}

impl Assignable<Value> for ValueSymbol {
    fn assignable(&self, _value: &Value) -> bool {
        // TODO: 実装

        true
    }
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
    Reference(ValueSymbolReference),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueSymbolReference(ValueSymbolName);

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
pub enum TransitionEffect {
    Id,
    Empty,
    StateLabel(StateLabel),
}

// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub struct transitionEffectLiteral(String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Value {
    Id,
    Empty,
    Effect(Effect),
    Context(ContextLabel),
    ContextEffect(ContextEffect),
    Slice(Slice),
    Snapshot(SnapshotValue),
    Scope(ScopeValue),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Effect {
    Id,
    Empty,
    AddEffect(AddEffect),
    TransitionEffect(TransitionEffect),
}

impl Effect {
    pub fn compose(&self, rhs: &Effect) -> Effect {
        match (self, rhs) {
            (Effect::AddEffect(lhs), rhs) => lhs.compose(rhs),
            (Effect::TransitionEffect(lhs), rhs) => lhs.compose(rhs),
            (Effect::Id, rhs) => rhs.clone(),
            _ => Effect::Empty,
        }
    }

    pub fn apply(&self, value: &SnapshotValueItemValue) -> SnapshotValueItemValue {
        value.apply(self)
    }
}

impl AddEffect {
    pub fn compose(&self, rhs: &Effect) -> Effect {
        match (self, rhs) {
            (AddEffect(lhs), Effect::AddEffect(AddEffect(rhs))) => {
                Effect::AddEffect(AddEffect(lhs + rhs))
            }
            (AddEffect(lhs), Effect::Id) => Effect::AddEffect(AddEffect(lhs.clone())),
            _ => Effect::Empty,
        }
    }
}

impl TransitionEffect {
    pub fn compose(&self, rhs: &Effect) -> Effect {
        match (self, rhs) {
            (
                TransitionEffect::StateLabel(_lhs),
                Effect::TransitionEffect(TransitionEffect::StateLabel(rhs)),
            ) => Effect::TransitionEffect(TransitionEffect::StateLabel(rhs.clone())),
            (TransitionEffect::StateLabel(lhs), Effect::Id) => {
                Effect::TransitionEffect(TransitionEffect::StateLabel(lhs.clone()))
            }
            _ => unimplemented!(), // _ => Effect::Empty,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slice(Vec<ContextEffect>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextEffect(ContextLabel, Effect);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment(String);

impl Expr {
    pub fn eval(&self, vm: &Vm) -> (Vm, Value) {
        match self {
            Expr::Id(v) => v.eval(vm),
            Expr::Op(op, lhs, rhs) => {
                let (vm, lhs_value) = lhs.eval(vm);
                let (vm, rhs_value) = rhs.eval(&vm);

                (vm.clone(), op.apply(lhs_value, rhs_value))
            }
            Expr::Reference(reference) => (
                vm.clone(),
                vm.lookup_value_symbol(reference).unwrap().1.clone(),
            ),
        }
    }
}

impl Value {
    pub fn eval(&self, vm: &Vm) -> (Vm, Value) {
        match self {
            Value::Id => (vm.clone(), Value::Id),
            Value::Empty => (vm.clone(), Value::Empty),
            Value::Effect(value) => {
                let (vm, value) = value.eval(vm);

                (vm, Value::Effect(value))
            }
            Value::Context(value) => {
                let (vm, value) = value.eval(vm);

                (vm, Value::Context(value))
            }
            Value::ContextEffect(value) => {
                let (vm, value) = value.eval(vm);

                (vm, Value::ContextEffect(value))
            }
            Value::Slice(value) => {
                let (vm, value) = value.eval(vm);

                (vm, Value::Slice(value))
            }
            Value::Snapshot(value) => {
                let (vm, value) = value.eval(vm);

                (vm, Value::Snapshot(value))
            }
            Value::Scope(value) => value.eval(vm),
        }
    }
}

impl Effect {
    pub fn eval(&self, vm: &Vm) -> (Vm, Effect) {
        return (vm.clone(), self.clone());
    }
}

impl ContextLabel {
    pub fn eval(&self, vm: &Vm) -> (Vm, ContextLabel) {
        return (vm.clone(), self.clone());
    }
}

impl ContextEffect {
    pub fn eval(&self, vm: &Vm) -> (Vm, ContextEffect) {
        match self {
            ContextEffect(label, effect) => {
                let (vm, label) = label.eval(vm);
                let (vm, effect) = effect.eval(&vm);

                (vm, ContextEffect(label, effect))
            }
        }
    }
}

impl Slice {
    pub fn eval(&self, vm: &Vm) -> (Vm, Slice) {
        let (vm, vec) = self
            .0
            .iter()
            .fold((vm.clone(), Vec::new()), |(vm, mut acc), item| {
                let (vm, value) = item.eval(&vm);
                acc.push(value);

                (vm, acc)
            });

        (vm, Slice(vec))
    }
}

impl SnapshotValue {
    pub fn eval(&self, vm: &Vm) -> (Vm, SnapshotValue) {
        let (vm, vec) = self
            .0
            .iter()
            .fold((vm.clone(), Vec::new()), |(vm, mut acc), item| {
                let (vm, value) = item.eval(&vm);
                acc.push(value);

                (vm, acc)
            });

        (vm, SnapshotValue(vec))
    }
}

impl SnapshotValueItem {
    pub fn eval(&self, vm: &Vm) -> (Vm, SnapshotValueItem) {
        match self {
            SnapshotValueItem(label, value) => {
                let (vm, label) = label.eval(vm);
                let (vm, value) = value.eval(&vm);

                (vm, SnapshotValueItem(label, value))
            }
        }
    }
}

impl SnapshotValueItemValue {
    pub fn eval(&self, vm: &Vm) -> (Vm, SnapshotValueItemValue) {
        match self {
            SnapshotValueItemValue::Id => (vm.clone(), SnapshotValueItemValue::Id),
            SnapshotValueItemValue::Empty => (vm.clone(), SnapshotValueItemValue::Empty),
            SnapshotValueItemValue::StateLabel(label) => {
                let (vm, label) = label.eval(vm);

                (vm, SnapshotValueItemValue::StateLabel(label))
            }
            SnapshotValueItemValue::Int(value) => {
                let (vm, value) = value.eval(vm);

                (vm, SnapshotValueItemValue::Int(value))
            }
        }
    }
}

impl StateLabel {
    pub fn eval(&self, vm: &Vm) -> (Vm, StateLabel) {
        (vm.clone(), self.clone())
    }
}

impl IntLiteral {
    pub fn eval(&self, vm: &Vm) -> (Vm, IntLiteral) {
        (vm.clone(), self.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VmScope {
    // TODO: VecをSymbolTableにする
    type_symbols: Vec<(TypeSymbol, TypeValue)>,
    value_symbols: Vec<(ValueSymbol, Value)>,
    return_value: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vm {
    stack: Vec<VmScope>,
}

impl Vm {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push_stack(&mut self) {
        self.stack.push(VmScope::new())
    }

    pub fn pop_stack(&mut self) -> Option<VmScope> {
        self.stack.pop()
    }

    pub fn current_scope(&mut self) -> &mut VmScope {
        self.stack.last_mut().unwrap()
    }

    pub fn lookup_value_symbol(
        &self,
        reference: &ValueSymbolReference,
    ) -> Option<&(ValueSymbol, Value)> {
        let scope = self
            .stack
            .iter()
            .rev()
            .find(|item| item.find_value_symbol(reference).is_some());

        match scope {
            Some(scope) => scope.find_value_symbol(reference),
            None => None,
        }
    }
}

impl VmScope {
    pub fn new() -> Self {
        Self {
            type_symbols: Vec::new(),
            value_symbols: Vec::new(),
            return_value: Value::Empty,
        }
    }

    pub fn push_type_symbol(&mut self, symbol: &TypeSymbol) {
        if self
            .type_symbols
            .iter()
            .find(|item| item.0.name.0 == symbol.name.0)
            .is_some()
        {
            panic!()
        }

        self.type_symbols.push((symbol.clone(), TypeValue::Empty));
    }

    pub fn assign_type_symbol(&mut self, symbol: &TypeSymbol, value: &TypeValue) {
        if !symbol.assignable(value) {
            panic!()
        }

        if let Some(sym) = self
            .type_symbols
            .iter_mut()
            .find(|item| item.0.name.0 == symbol.name.0)
        {
            sym.1 = value.clone();
        } else {
            self.type_symbols.push((symbol.clone(), value.clone()));
        }
    }

    pub fn push_value_symbol(&mut self, symbol: &ValueSymbol) {
        if self
            .value_symbols
            .iter()
            .find(|item| item.0.name.0 == symbol.name.0)
            .is_some()
        {
            panic!()
        }

        self.value_symbols.push((symbol.clone(), Value::Empty));
    }

    pub fn assign_value_symbol(&mut self, symbol: &ValueSymbol, value: &Value) {
        if !symbol.assignable(value) {
            panic!()
        }

        if let Some(sym) = self
            .value_symbols
            .iter_mut()
            .find(|item| item.0.name.0 == symbol.name.0)
        {
            sym.1 = value.clone();
        } else {
            self.value_symbols.push((symbol.clone(), value.clone()));
        }
    }

    pub fn push_return_value(&mut self, value: &Value) {
        self.return_value = value.clone()
    }

    pub fn return_value(&self) -> Value {
        self.return_value.clone()
    }

    pub fn find_value_symbol(
        &self,
        reference: &ValueSymbolReference,
    ) -> Option<&(ValueSymbol, Value)> {
        self.value_symbols.iter().find(
            |(
                ValueSymbol {
                    name: ValueSymbolName(sym_name),
                    ..
                },
                _,
            )| match reference {
                ValueSymbolReference(ValueSymbolName(ref_name)) => sym_name == ref_name,
            },
        )
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

impl LetExpr {
    pub fn eval(&self, vm: &Vm) -> (Vm, Value) {
        match self {
            Self::Bind(symbol) => {
                let mut vm = vm.clone();
                vm.current_scope().push_value_symbol(&symbol);

                (vm.clone(), Value::Empty)
            }
            Self::Assign { symbol, expr } => {
                let (mut vm, value) = expr.eval(&vm);
                vm.current_scope().assign_value_symbol(symbol, &value);

                (vm.clone(), value.clone())
            }
        }
    }
}

impl ScopeInnerValue {
    pub fn eval(&self, vm: &Vm) -> (Vm, Value) {
        match self {
            ScopeInnerValue::Comment(_) => (vm.clone(), Value::Empty),
            ScopeInnerValue::Stmt(value) => (value.eval(vm), Value::Empty),
            ScopeInnerValue::Expr(value) => {
                let (mut vm, value) = value.eval(vm);
                vm.current_scope().push_return_value(&value);

                (vm, value)
            }
        }
    }
}

impl Stmt {
    pub fn eval(&self, vm: &Vm) -> Vm {
        let vm = match self {
            Stmt::Expr(expr) => {
                let (vm, _) = expr.eval(vm);

                vm
            }
            Stmt::TypeStmt(expr) => {
                let (vm, _) = expr.eval(vm);

                vm
            }
            Stmt::LetStmt(expr) => {
                let (vm, _) = expr.eval(vm);

                vm
            }
            Stmt::ScopeStmt(scope) => {
                let (vm, _) = scope.eval(vm);

                vm
            }
        };

        vm
    }
}

impl ScopeValue {
    pub fn eval(&self, vm: &Vm) -> (Vm, Value) {
        let mut vm = vm.clone();
        vm.push_stack();

        let (mut vm, value) = self
            .0
            .iter()
            .fold((vm, Value::Empty), |(vm, _value), stmt| stmt.eval(&vm));

        // let scope = vm.pop_stack();
        vm.pop_stack();

        (vm, value)
    }
}

impl Op {
    pub fn apply(&self, lhs: Value, rhs: Value) -> Value {
        match (self, &lhs, &rhs) {
            (Op::Compose, lhs, rhs) => eval_compose(lhs, rhs),
            (Op::Apply, lhs, rhs) => eval_apply(lhs, rhs),
            (Op::Reduce, lhs, rhs) => eval_reduce(lhs, rhs),
            (Op::Push, lhs, rhs) => eval_push(lhs, rhs),
        }
    }
}

fn eval_compose(lhs: &Value, rhs: &Value) -> Value {
    match (lhs, rhs) {
        (Value::Id, Value::Id) => Value::Id,
        (any, Value::Id) => any.clone(),
        (Value::Id, any) => any.clone(),

        (Value::Effect(lhs), Value::Effect(rhs)) => Value::Effect(lhs.compose(rhs)),
        (Value::Slice(_), Value::Slice(_)) => unimplemented!(),
        (Value::ContextEffect(_), Value::ContextEffect(_)) => unimplemented!(),

        _ => Value::Empty,
    }
}

fn eval_push(lhs: &Value, rhs: &Value) -> Value {
    match (lhs, rhs) {
        (Value::Slice(slice), Value::ContextEffect(context_effect)) => {
            let mut slice = slice.clone();
            slice.0.push(context_effect.clone());

            Value::Slice(slice)
        }
        _ => Value::Empty,
    }
}

fn eval_reduce(lhs: &Value, rhs: &Value) -> Value {
    match (lhs, rhs) {
        (Value::Slice(slice), Value::Snapshot(snapshot)) => Value::Snapshot(slice.apply(snapshot)),
        _ => Value::Empty,
    }
}

impl Slice {
    pub fn apply(&self, snapshot: &SnapshotValue) -> SnapshotValue {
        let context_effects = self
            .0
            .iter()
            .fold(
                HashMap::<ContextLabel, Vec<Effect>>::new(),
                |mut acc, ContextEffect(label, effect)| {
                    let effects = match acc.get_mut(label) {
                        Some(v) => v,
                        None => {
                            acc.insert(label.clone(), Vec::new());
                            acc.get_mut(label).unwrap()
                        }
                    };
                    effects.push(effect.clone());

                    acc
                },
            )
            .iter()
            .map(|(label, effects)| {
                ContextEffect(
                    label.clone(),
                    effects
                        .iter()
                        .fold(Effect::Id, |acc, effect| acc.compose(effect)),
                )
            })
            .fold(snapshot.clone(), |acc, ContextEffect(label, effect)| {
                let value = match snapshot.find_by_label(&label) {
                    Some(SnapshotValueItem(_, value)) => effect.apply(value),
                    _ => SnapshotValueItemValue::Empty,
                };

                acc.insert(&label, &value)
            });

        context_effects
    }
}

fn eval_apply(lhs: &Value, rhs: &Value) -> Value {
    match (lhs, rhs) {
        (Value::Context(label), Value::Effect(effect)) => {
            Value::ContextEffect(ContextEffect(label.clone(), effect.clone()))
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
        Rule::typeStmt => Stmt::TypeStmt(parse_type_expr(&pair)),
        Rule::letStmt => Stmt::LetStmt(parse_let_expr(&pair)),
        Rule::calcStmt => Stmt::Expr(parse_expr(&pair)),
        Rule::scopeStmt => Stmt::ScopeStmt(parse_scope_stmt(&pair)),
        _ => panic!(),
    }
}

fn parse_let_expr(pair: &Pair<'_, Rule>) -> LetExpr {
    let pair = pair.clone().into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::bindExpr => LetExpr::Bind(parse_bind_expr(&pair)),
        Rule::assignExpr => parse_assign_expr(&pair),
        _ => panic!(),
    }
}

fn parse_assign_expr(pair: &Pair<'_, Rule>) -> LetExpr {
    match inner_len(pair) {
        2 => {
            let a = pair
                .clone()
                .into_inner()
                .map(|item| item)
                .collect::<Vec<_>>();
            let mut a = a.iter();

            let lhs = a.next().unwrap();
            let rhs = a.next().unwrap();

            LetExpr::Assign {
                symbol: parse_bind_expr(lhs),
                expr: parse_expr(rhs),
            }
        }
        _ => panic!(),
    }
}

fn parse_bind_expr(pair: &Pair<'_, Rule>) -> ValueSymbol {
    match inner_len(pair) {
        2 => {
            let a = pair
                .clone()
                .into_inner()
                .map(|item| item)
                .collect::<Vec<_>>();
            let mut a = a.iter();

            let lhs = a.next().unwrap();
            let rhs = a.next().unwrap();

            ValueSymbol {
                name: parse_var_symbol(lhs),
                kind: parse_bind_keywords(rhs),
            }
        }
        _ => panic!(),
    }
}

fn parse_bind_keywords(pair: &Pair<'_, Rule>) -> BindKeywords {
    match pair.as_span().as_str() {
        "Slice" => BindKeywords::Slice,
        "Effect" => BindKeywords::Effect,
        "ContextEffect" => BindKeywords::ContextEffect,
        "Context" => BindKeywords::Context,
        "Snapshot" => BindKeywords::Snapshot,
        _ => unimplemented!(),
    }
}

fn inner_len(pair: &Pair<'_, Rule>) -> usize {
    pair.clone()
        .into_inner()
        .map(|_item| true)
        .collect::<Vec<_>>()
        .len()
}

fn parse_type_expr(pair: &Pair<'_, Rule>) -> TypeExpr {
    let pair = pair.clone().into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::bindTypeExpr => TypeExpr::Bind(parse_type_bind_expr(&pair)),
        Rule::assignTypeExpr => parse_type_assign_expr(&pair),
        Rule::typeExpr => {
            unimplemented!()
        }
        _ => panic!(),
    }
}

fn parse_type_bind_expr(pair: &Pair<'_, Rule>) -> TypeSymbol {
    match inner_len(pair) {
        2 => {
            let a = pair
                .clone()
                .into_inner()
                .map(|item| item)
                .collect::<Vec<_>>();
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

fn parse_type_assign_expr(pair: &Pair<'_, Rule>) -> TypeExpr {
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
                symbol: parse_type_bind_expr(lhs),
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
                Rule::contextedTypeExpr => TypeValue::Context(parse_context_type_expr(&pair)),
                Rule::typeLiteral => TypeValue::Context(parse_context_type_expr(&pair)),
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

fn parse_snapshot_type_expr(pair: &Pair<'_, Rule>) -> SnapshotType {
    match pair.as_rule() {
        Rule::snapshotTypeExpr => {
            let pair = pair.clone().into_inner().next().unwrap();

            parse_snapshot_type_literal(&pair)
        }
        _ => unimplemented!(),
    }
}

fn parse_snapshot_type_literal(pair: &Pair<'_, Rule>) -> SnapshotType {
    // TODO: & によるStateMachineの合成がある。あと、途中に式が挟まることがある
    match pair.as_rule() {
        Rule::snapshotTypeLiteral => {
            let contexts = pair
                .clone()
                .into_inner()
                .map(|item| parse_snapshot_type_item_literal(&item))
                .collect::<Vec<_>>();

            SnapshotType(contexts)
        }
        _ => unimplemented!(),
    }
}

fn parse_snapshot_type_item_literal(pair: &Pair<'_, Rule>) -> ContextType {
    match inner_len(pair) {
        1 => unimplemented!(),
        2 => {
            let a = pair
                .clone()
                .into_inner()
                .map(|item| item)
                .collect::<Vec<_>>();
            let mut a = a.iter();

            let lhs = a.next().unwrap();
            let rhs = a.next().unwrap();

            ContextType(parse_context_label(lhs), parse_effect_type_expr(rhs))
        }
        _ => panic!(),
    }
}

fn parse_context_label(pair: &Pair<'_, Rule>) -> ContextLabel {
    ContextLabel(parse_var_symbol(pair).0)
}

fn parse_var_symbol(pair: &Pair<'_, Rule>) -> ValueSymbolName {
    match pair.as_rule() {
        Rule::varSymbol => ValueSymbolName(pair.as_span().as_str().to_string()),
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
                .map(|item| parse_state_expr(&item))
                .collect::<Vec<_>>();

            StateMachine(states)
        }
        _ => unimplemented!(),
    }
}

fn parse_state_expr(pair: &Pair<'_, Rule>) -> StateLabel {
    let pair = pair.clone().into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::stateLiteral => parse_state_literal(&pair),
        Rule::idLiteral => StateLabel::Id,
        Rule::emptyLiteral => StateLabel::Empty,
        _ => unimplemented!(),
    }
}

fn parse_state_literal(pair: &Pair<'_, Rule>) -> StateLabel {
    let pair = pair.clone().into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::likeString => StateLabel::Label(pair.as_span().as_str().to_string()),
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
        Rule::contextKeyword => TypeKind::Context,
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
    match inner_len(pair) {
        1 => {
            let v = pair.clone().into_inner().next().unwrap();
            match v.as_rule() {
                Rule::term => parse_term(&v),
                Rule::expr => parse_expr(&v),
                _ => panic!(),
            }
        }
        3 => {
            let a = pair
                .clone()
                .into_inner()
                .map(|item| item)
                .collect::<Vec<_>>();
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
    let pair = pair.clone().into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::expr => parse_expr(&pair),
        Rule::varSymbol => Expr::Reference(ValueSymbolReference(parse_var_symbol(&pair))),
        Rule::addEffectLiteral => Expr::Id(Value::Effect(Effect::AddEffect(
            parse_add_effect_literal(&pair),
        ))),
        Rule::transitionEffectLiteral => Expr::Id(Value::Effect(Effect::TransitionEffect(
            parse_transition_effect_literal(&pair),
        ))),
        Rule::idLiteral => Expr::Id(Value::Id),
        Rule::emptyLiteral => Expr::Id(Value::Empty),
        Rule::sliceLiteral => Expr::Id(Value::Slice(parse_slice_expr(&pair))),
        Rule::snapshotLiteral => Expr::Id(Value::Snapshot(parse_snapshot_value_expr(&pair))),
        Rule::contextEffectLiteral => {
            Expr::Id(Value::ContextEffect(parse_context_effect_expr(&pair)))
        }
        Rule::contextTypeLiteral => {
            let a = parse_context_type_expr(&pair);
            Expr::Id(Value::Context(a.0.clone()))
        }
        Rule::effectLiteral => Expr::Id(Value::Effect(parse_effect_literal(&pair))),
        Rule::transitionEffectExpr => Expr::Id(Value::Effect(Effect::TransitionEffect(
            parse_transition_effect_expr(&pair),
        ))),
        Rule::scopeExpr => Expr::Id(Value::Scope(parse_scope_expr(&pair))),
        _ => panic!(),
    }
}

fn parse_effect_literal(pair: &Pair<'_, Rule>) -> Effect {
    let pair = pair.clone().into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::idLiteral => Effect::Id,
        Rule::emptyLiteral => Effect::Empty,
        Rule::addEffectExpr => Effect::AddEffect(parse_add_effect_expr(&pair)),
        Rule::transitionEffectExpr => Effect::TransitionEffect(parse_transition_effect_expr(&pair)),
        _ => panic!(),
    }
}

fn parse_context_type_expr(pair: &Pair<'_, Rule>) -> ContextType {
    match inner_len(pair) {
        1 => {
            let pair = pair.clone().into_inner().next().unwrap();
            parse_context_type_expr(&pair)
        }
        2 => {
            let a = pair
                .clone()
                .into_inner()
                .map(|item| item)
                .collect::<Vec<_>>();
            let mut a = a.iter();

            let lhs = a.next().unwrap();
            let rhs = a.next().unwrap();

            ContextType(parse_context_label(lhs), parse_effect_type_expr(rhs))
        }
        _ => unimplemented!(),
    }
}

fn parse_snapshot_value_expr(pair: &Pair<'_, Rule>) -> SnapshotValue {
    match pair.as_rule() {
        Rule::snapshotLiteral => parse_snapshot_value_literal(pair),
        _ => unimplemented!(),
    }
}

fn parse_snapshot_value_literal(pair: &Pair<'_, Rule>) -> SnapshotValue {
    let a = pair
        .clone()
        .into_inner()
        .map(|item| parse_snapshot_value_item_expr(&item))
        .collect::<Vec<_>>();

    SnapshotValue(a)
}

fn parse_snapshot_value_item_expr(pair: &Pair<'_, Rule>) -> SnapshotValueItem {
    match pair.as_rule() {
        Rule::snapshotItemLiteral => parse_snapshot_value_item_literal(pair),
        _ => unimplemented!(),
    }
}

fn parse_snapshot_value_item_literal(pair: &Pair<'_, Rule>) -> SnapshotValueItem {
    match inner_len(pair) {
        2 => {
            let a = pair
                .clone()
                .into_inner()
                .map(|item| item)
                .collect::<Vec<_>>();
            let mut a = a.iter();

            let lhs = a.next().unwrap();
            let rhs = a.next().unwrap();

            SnapshotValueItem(
                parse_context_expr(lhs),
                parse_snapshot_value_item_value_expr(rhs),
            )
        }
        _ => panic!(),
    }
}

fn parse_snapshot_value_item_value_expr(pair: &Pair<'_, Rule>) -> SnapshotValueItemValue {
    match pair.as_rule() {
        Rule::snapshotValueExpr => {
            let pair = pair.clone().into_inner().next().unwrap();
            parse_snapshot_value_item_value_expr(&pair)
        }
        Rule::snapshotValueLiteral => parse_snapshot_value_item_value_literal(pair),
        _ => unimplemented!(),
    }
}

fn parse_snapshot_value_item_value_literal(pair: &Pair<'_, Rule>) -> SnapshotValueItemValue {
    let pair = pair.clone().into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::intLiteral => SnapshotValueItemValue::Int(parse_int_literal(&pair)),
        Rule::stateExpr => SnapshotValueItemValue::StateLabel(parse_state_expr(&pair)),
        Rule::idLiteral => SnapshotValueItemValue::Id,
        Rule::emptyLiteral => SnapshotValueItemValue::Empty,
        _ => unimplemented!(),
    }
}

fn parse_slice_expr(pair: &Pair<'_, Rule>) -> Slice {
    parse_slice_literal(pair)
}

fn parse_slice_literal(pair: &Pair<'_, Rule>) -> Slice {
    let a = pair
        .clone()
        .into_inner()
        .map(|item| parse_context_effect_expr(&item))
        .collect::<Vec<_>>();

    Slice(a)
}

fn parse_context_effect_expr(pair: &Pair<'_, Rule>) -> ContextEffect {
    match inner_len(pair) {
        1 => {
            let pair = pair.clone().into_inner().next().unwrap();
            parse_context_effect_literal(&pair)
        }
        2 => parse_context_effect_literal(&pair),
        _ => panic!(),
    }
}

fn parse_context_effect_literal(pair: &Pair<'_, Rule>) -> ContextEffect {
    match inner_len(pair) {
        2 => {
            let a = pair
                .clone()
                .into_inner()
                .map(|item| item)
                .collect::<Vec<_>>();
            let mut a = a.iter();

            let lhs = a.next().unwrap();
            let rhs = a.next().unwrap();

            ContextEffect(parse_context_expr(lhs), parse_effect_expr(rhs))
        }
        _ => panic!(),
    }
}

fn parse_context_expr(pair: &Pair<'_, Rule>) -> ContextLabel {
    match pair.as_rule() {
        Rule::contextExpr => {
            let pair = pair.clone().into_inner().next().unwrap();
            parse_context_type_literal(&pair)
        }
        _ => unimplemented!(),
    }
}

fn parse_context_type_literal(pair: &Pair<'_, Rule>) -> ContextLabel {
    match pair.as_rule() {
        Rule::varSymbol => ContextLabel(parse_var_symbol(&pair).0),
        Rule::contextLiteral => {
            let pair = pair.clone().into_inner().next().unwrap();
            parse_context_type_literal(&pair)
        }
        Rule::contextTypeLiteral => {
            let pair = pair.clone().into_inner().next().unwrap();
            ContextLabel(parse_context_label(&pair).0)
        }
        _ => {
            unimplemented!();
        }
    }
}

fn parse_effect_expr(pair: &Pair<'_, Rule>) -> Effect {
    match pair.as_rule() {
        Rule::addEffectLiteral => {
            let pair = pair.clone().into_inner().next().unwrap();
            Effect::AddEffect(parse_add_effect_literal(&pair))
        }
        Rule::transitionEffectLiteral => {
            let pair = pair.clone().into_inner().next().unwrap();
            Effect::TransitionEffect(parse_transition_effect_literal(&pair))
        }
        Rule::idLiteral => Effect::Id,
        Rule::emptyLiteral => Effect::Empty,
        Rule::effectExpr => {
            let pair = pair.clone().into_inner().next().unwrap();
            parse_effect_literal(&pair)
        }
        _ => unimplemented!(),
    }
}

fn parse_add_effect_expr(pair: &Pair<'_, Rule>) -> AddEffect {
    let pair = pair.clone().into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::addEffectLiteral => {
            let pair = pair.clone().into_inner().next().unwrap();
            parse_add_effect_literal(&pair)
        }
        _ => unimplemented!(),
    }
}

fn parse_add_effect_literal(pair: &Pair<'_, Rule>) -> AddEffect {
    match pair.as_rule() {
        Rule::addEffectLiteral => {
            let pair = pair.clone().into_inner().next().unwrap();
            parse_add_effect_literal(&pair)
        }
        Rule::intLiteral => AddEffect(parse_int_literal(&pair).0),
        _ => unimplemented!(),
    }
}

fn parse_int_literal(pair: &Pair<'_, Rule>) -> IntLiteral {
    IntLiteral(pair.as_span().as_str().parse::<i64>().unwrap())
}

fn parse_transition_effect_expr(pair: &Pair<'_, Rule>) -> TransitionEffect {
    let pair = pair.clone().into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::idLiteral => TransitionEffect::Id,
        Rule::emptyLiteral => TransitionEffect::Empty,
        Rule::transitionEffectLiteral => parse_transition_effect_literal(&pair),
        _ => unimplemented!(),
    }
}

fn parse_transition_effect_literal(pair: &Pair<'_, Rule>) -> TransitionEffect {
    let pair = pair.clone().into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::stateExpr => TransitionEffect::StateLabel(parse_state_expr(&pair)),
        _ => unimplemented!(),
    }
}

fn parse_comment(pair: &Pair<'_, Rule>) -> Comment {
    Comment(pair.as_span().as_str().to_string())
}

fn parse_scope_inner(pair: &Pair<'_, Rule>) -> ScopeValue {
    let items = pair
        .clone()
        .into_inner()
        .map(|item| item)
        .collect::<Vec<_>>();
    let pos = items.iter().position(|item| item.as_rule() == Rule::expr);
    if pos.is_none() {
        panic!()
    }

    let ret = items
        .iter()
        .map(|item| match item.as_rule() {
            Rule::comment => ScopeInnerValue::Comment(parse_comment(item)),
            Rule::stmt => ScopeInnerValue::Stmt(parse_stmt(item)),
            Rule::expr => ScopeInnerValue::Expr(parse_expr(item)),
            _ => {
                unimplemented!()
            }
        })
        .collect::<_>();

    ScopeValue(ret)
}

fn parse_scope_literal(pair: &Pair<'_, Rule>) -> ScopeValue {
    let pair = pair.clone().into_inner().next().unwrap();

    parse_scope_inner(&pair)
}

fn parse_scope_expr(pair: &Pair<'_, Rule>) -> ScopeValue {
    let pair = pair.clone().into_inner().next().unwrap();

    parse_scope_literal(&pair)
}

fn parse_scope_stmt(pair: &Pair<'_, Rule>) -> ScopeValue {
    let pair = pair.clone().into_inner().next().unwrap();

    parse_scope_expr(&pair)
}

fn parse_root_scope(pair: &Pair<'_, Rule>) -> ScopeValue {
    let pair = pair.clone().into_inner().next().unwrap();

    parse_scope_inner(&pair)
}

pub fn to_node(pair: &Pair<'_, Rule>) -> Node {
    match pair.as_rule() {
        Rule::document => {
            let pair = pair.clone().into_inner().next().unwrap();
            Node::Root(parse_root_scope(&pair))
        }
        // Rule::EOI => Node::Comment(Comment("".to_string())),
        _ => {
            unimplemented!();
        }
    }
}
