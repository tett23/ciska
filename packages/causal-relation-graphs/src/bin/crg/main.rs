extern crate causal_relation_graphs;

use std::env;
use std::fs;

fn main() {
    let args = env::args();
    let filename = args.last().expect("a");
    let file = fs::read_to_string(filename).expect("");

    let result = causal_relation_graphs::run(&file);
    match result {
        Ok(result) => println!("{}", result),
        Err(err) => println!("error, {}", err),
    }

    ()
}
