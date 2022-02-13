extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;

use parser::{Node, Value, Vm};

pub fn parse(document: &str) -> Result<Node, String> {
    parser::parse(document)
}

pub fn run(document: &str) -> Result<String, String> {
    execute_ast(&parse(document)?)
}

pub fn execute_ast(ast: &Node) -> Result<String, String> {
    match ast {
        Node::Scope(_) => (),
        _ => return Err("".to_string()),
    }

    let mut vm = Vm::new();
    let value = match ast {
        Node::Scope(scope) => scope.eval(&mut vm),
        _ => Value::Empty,
    };

    serde_json::to_string_pretty(&value).map_err(|err| err.to_string())
}

// #[derive(Debug, Clone)]
// pub struct Action<T> {
//     id: String,
//     label: T,
//     name: String,
//     effect: Effects,
// }

// trait Label {}

// #[derive(Debug, Clone)]
// enum ActionLabels {
//     AppendableLable,
//     StateLabel,
// }

// impl<T> Action<T> {
//     pub fn new_add(label: T) -> Self {
//         Self {
//             id: "1".to_string(),
//             label,
//             name: "1".to_string(),
//             effect: Effects::new_add(),
//         }
//     }

//     pub fn new_transaction(label: T) -> Self {
//         Self {
//             id: "1".to_string(),
//             label,
//             name: "1".to_string(),
//             effect: Effects::new_transaction(),
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub enum Effects {
//     AddEffect { value: usize },
//     TransitionEffect(TransitionEffect),
//     IdEffect,
//     EmptyEffect,
// }

// impl Effects {
//     pub fn new_add() -> Self {
//         Effects::AddEffect { value: 1 }
//     }

//     pub fn new_transaction() -> Self {
//         Effects::AddEffect { value: 1 }
//     }
// }

// #[derive(Debug, Clone)]
// pub enum EffectValue<T> {
//     Id,
//     Empty,
//     Value { value: T },
// }

// #[derive(Debug, Clone)]
// pub struct AddEffect {}
// #[derive(Debug, Clone)]
// pub struct TransitionEffect {}

// #[derive(Debug, Clone)]
// pub struct Transaction<T>
// where
//     T: TransactionStatusable,
// {
//     actions: Vec<Action<T>>,
// }

// impl<T> Transaction<T>
// where
//     T: Clone,
// {
//     pub fn new() -> Self {
//         Self { actions: vec![] }
//     }

//     pub fn to_slice(&self) -> TransactionSlice<T> {
//         TransactionSlice {
//             actions: self.actions.clone(),
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub enum Slices<T> {
//     Slice { slice: TransactionSlice<T> },
//     Id,
//     Empty,
// }

// #[derive(Debug, Clone)]
// pub struct TransactionSlice<T> {
//     actions: Vec<Action<T>>,
// }

// impl<T> TransactionSlice<T>
// where
//     T: Clone,
// {
//     pub fn insertAfter(&self, action: &Action<T>) -> Result<TransactionSlice<T>, String> {
//         let mut a = self.actions.clone();
//         a.append(&mut vec![action.clone()]);

//         Ok(TransactionSlice { actions: a })
//     }

//     pub fn apply(&self, init: T) -> T {
//         self.actions.iter().fold(init, |acc, action| {
//             // match acc.
//             let state = acc.findByLabel(&action.label);
//             // action.effect

//             // action.
//             acc
//         });

//         init
//     }
// }

// trait TransactionStatusable {
//     fn applyValue<T>(&self, label: T) -> Self {}
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[derive(Debug, Clone)]
//     enum State1 {
//         Start,
//         End,
//     }

//     #[derive(Debug, Clone)]
//     struct State {
//         amount: (i32, ActionLabels),
//         state1: (State1, ActionLabels),
//     }
//     // impl Default for State {}

//     #[test]
//     fn it_works() {
//         let a = || -> Result<State, String> {
//             let transaction = Transaction::<State>::new();
//             let result = transaction
//                 .to_slice()
//                 .insertAfter(&Action::new_add(ActionLabels::AppendableLable))?
//                 .insertAfter(&Action::new_add(ActionLabels::AppendableLable))?
//                 .apply(State {
//                     amount: (1, ActionLabels::AppendableLable),
//                     state1: (State1::Start, ActionLabels::StateLabel),
//                 });
//             dbg!(&result);

//             result
//         }();
//         assert!(a.is_ok());

//         ()
//     }
// }
