extern crate causal_relation_graphs;

use insta::assert_yaml_snapshot;
use std::{fs, path::Path};

#[test]
fn parse_empty() {
    let text = r###"
type Foo :: StateMachine = (Id => a) & (Id => b) & Foo;
type Foo :: StateMachine = (Id => a) & (Id => b) & Foo;
type Foo :: StateMachine = (Id => a);
type Foo :: Snapshot = { foo: Int, a, a: Id => a };
type Foo :: StateMachine;
type Foo :: StateMachine = Id => a => b;
type Foo :: StateMachine = Id => a => b;
type Foo :: Context = a: Int;
type Foo :: Snapshot = {  };
type Foo :: Snapshot = {  };
a reduce {};
[] << (a, +1);
let s :: Snapshot = a reduce {};
{};
{a: 1};
{a:Int: 1};
//{(a:Int): 1};
{a: 1, a: 2};
let _a :: Snapshot;
let _a :: Snapshot = {a: 1, b: hoge, c: 2};
let a :: Context;
let a :: Context = a:Int;
let a :: ContextEffect = (a, +1);
let d :: Effect;
let d :: Effect = +1;
let a :: Slice;
let a :: Slice = [(a, +1)];
[(a, +1)];
[(a: Int, +1)];
[(a, +1), (b, =>a)];
+1;
[];
+1;
(+1);
+1 compose +1;
let a :: Slice;
let a :: Slice = +1;
let a :: Slice = [];
(foo: Int apply +1) compose +1;
(+1 compose +1) compose (+1 compose +1);
+1 compose (+1 compose (+1 compose +1));
a apply +1;
+1 compose +1 compose +1;
a apply (+10 compose +10);

=>a compose =>b;
Id compose Empty;
+1 compose +10;
+(-1) compose +1;
+1 compose =>a;

type A :: StateMachine = Id => start => a;
type A :: Snapshot = {};
type A :: Snapshot = {a};
type A :: StateMachine ;
type A :: StateMachine  = Id => start => end;
type Foo :: Context = label: Int;
type A :: Snapshot  = {foo: A, bar: B, baz: Int};
type A :: Snapshot = {foo: A, foo};
// Contextのリテラルを書ける
// (foo :: Int) apply +1

type A :: StateMachine;
type A :: StateMachine = Id => start => end;
type Context_a :: Context = foo: Int;
type B :: Snapshot = {foo: A, bar: B, baz: Int};

type AAA  :: Snapshot = { context_a, context_b, context_c: Int };

// ここはあとで実装する
// type Constraint A :: PositiveInt;
// VirtualActionはあとで実装する

hoge apply +10;
=>a compose =>b;
Id compose =>b;
+10 compose +10;
+10 compose +(10);
+10 compose +(-10);
hoge apply (+10 compose +10);
// (hoge apply (+10 compose +10)) compose (hoge apply (+10 compose +10));

// let a :: Slice;
// let a :: Slice = [];
// let a :: Slice = [(hoge, +1)] << (hoge, +1);
// ## << はPush演算で、左から解釈される。Pushされると新しいSliceを生成する
// a << (hoge, +1);
// let aa = push a (hoge, +10);
// push a (hoge, +10);
// push aa, a (hoge, +10);
// push a (bar, =>registrated);

// let a :: Slice;
// let c :: Slice = [a, a compose b];

// let snap1 Snapshot :: A;
// let snap2 Snapshot :: A & B;

// c reduce (snap1);

// // Outputで副作用を扱う構文をやりたい
// // 戻り値はEmpty
// Output << c reduce snap2;
// Output << (c reduce snap2);

// 戻り値の表示
// Return << c;
// Return << c;

    "###;

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
