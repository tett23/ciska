extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;

pub fn parse(document: &str) {
    let node = parser::parse(document);
    let json = serde_json::to_string_pretty(&node);
    let json = json.unwrap();
    println!("{}", &json);
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn it_works() {
        // parse("");
        // parse("\n");
        // parse("%% foo\n# hoge");
        // parse("# foo");
        // parse("## foo");
        // parse("# #foo");
        parse(
            r"
# foo

bar
%% block comment
#tag-name #tag2 paragraph text -%inline comment%
",
        );
        // parse("1\n2\n3");
    }
}
