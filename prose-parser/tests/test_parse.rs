extern crate prose_parser;

use insta::assert_yaml_snapshot;
use std::{fs, path::Path};

#[test]
fn parse_empty() {
    assert_yaml_snapshot!(prose_parser::parse("").unwrap(), @r###"
    ---
    name: document
    "###);
    assert_yaml_snapshot!(prose_parser::parse("1\n2\n3").unwrap(), @r###"
    ---
    name: document
    children:
      - name: paragraph
        children:
          - name: text
            value: "1"
      - name: line_break
      - name: paragraph
        children:
          - name: text
            value: "2"
      - name: line_break
      - name: paragraph
        children:
          - name: text
            value: "3"
    "###);
}

#[test]
fn parse2() {
    assert_yaml_snapshot!(prose_parser::parse("#tag word %comment% %comment foo% word2 #tag2").unwrap(), @r###"
    ---
    name: document
    children:
      - name: paragraph
        children:
          - name: tag
            children:
              - name: text
                value: tag
          - name: text
            value: "word "
          - name: inline_comment
            children:
              - name: text
                value: comment
          - name: text
            value: " "
          - name: inline_comment
            children:
              - name: text
                value: comment foo
          - name: text
            value: " word2 "
          - name: tag
            children:
              - name: text
                value: tag2
    "###);
}

#[test]
fn parse() {
    read_fixtures().iter().for_each(|(name, content)| {
        assert_yaml_snapshot!(
            name.as_str(),
            prose_parser::parse(content.as_str()).unwrap()
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
