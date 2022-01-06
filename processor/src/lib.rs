// #![feature(inherent_associated_types)]

mod parser;

use parser::{Parser, ParserProcessor};

pub enum Processor<N> {
    None,
    ParserProcessor(ParserProcessor<N>),
}

impl<N> Processor<N> {
    pub fn new() -> Processor<N> {
        Processor::None
    }

    pub fn parser<F>(&self, f: Box<dyn Parser<N>>) -> Processor<N>
    where
        F: Fn(&str) -> Result<N, String>,
    {
        Processor::ParserProcessor(ParserProcessor::new(f))
    }

    pub fn parse(&self, text: &str) -> Result<String, String> {
        match self {
            Processor::None => Err("".to_string()),
            Processor::ParserProcessor(processor) => Ok("".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let actual = Processor::<String>::new()
        //     .parser(|text: String| Err("".to_string()))
        //     .parse("");
        // assert_eq!(Err("".to_string()), actual);
    }
}
