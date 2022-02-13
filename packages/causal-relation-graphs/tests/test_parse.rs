extern crate causal_relation_graphs;

use insta::assert_yaml_snapshot;
use std::{fs, path::Path};

#[test]
fn parse_empty() {
    let text = r###""###;

    dbg!(text);

    assert_yaml_snapshot!(causal_relation_graphs::parse(text).unwrap(), @r###"
    ---
    name: document
    "###);
}

#[test]
fn parse() {
    read_fixtures().iter().for_each(|(name, content)| {
        assert_yaml_snapshot!(
            name.as_str(),
            causal_relation_graphs::parse(content.as_str()).unwrap()
        );
    });
}

#[test]
fn execute() {
    // let text =
    //     "+1 compose +2 compose +1; +1; // hoge\n=>a; Id; Empty; =>a compose =>b; Id compose =>a;";
    let text =
        "type A :: StateMachine; type B :: StateMachine = Id => a; type C :: StateMachine = {context_a: Int, context_b: B}; let a :: Effect; let b :: Effect = +1;";
    let result = causal_relation_graphs::execute(&causal_relation_graphs::parse(text).unwrap());

    dbg!(result);
}

fn read_fixtures() -> Vec<(String, String)> {
    fs::read_dir("./tests/fixtures")
        .unwrap()
        .filter_map(|item| {
            let ent = item.ok()?;

            match ent.file_type().ok()?.is_file() {
                true => Some(ent.path().to_string_lossy().into_owned()),
                false => None,
            }
        })
        .map(|item| match fs::read_to_string(&item) {
            Ok(ent) => Ok((
                Path::new(&item)
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                ent,
            )),
            Err(e) => Err(e),
        })
        .flat_map(|item| item)
        .collect::<_>()
}
