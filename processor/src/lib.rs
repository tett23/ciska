// #![feature(inherent_associated_types)]

mod parser;

use parser::{Parser, ParserProcessor};

pub enum Processor<N>
where
    N: 'static,
{
    None,
    ParserProcessor(ParserProcessor<N>),
}

impl<N> Processor<N> {
    pub fn new() -> Processor<N> {
        Processor::None
    }

    pub fn parser<F>(&self, f: F) -> Processor<N>
    where
        F: Fn(&str) -> Result<Box<N>, String> + 'static,
    {
        Processor::ParserProcessor(ParserProcessor::new(f.into()))
    }

    pub fn parse(&self, text: &str) -> Result<Box<N>, String> {
        match self {
            Processor::None => Err("".to_string()),
            Processor::ParserProcessor(processor) => processor.parse(text),
        }
    }
}

impl<F, R> From<F> for Box<dyn Parser<R>>
where
    F: Fn(&str) -> Result<Box<R>, String> + 'static,
    R: 'static,
{
    fn from(f: F) -> Self {
        into_parser(f)
    }
}

struct P2<F, R>
where
    F: Fn(String) -> Result<Box<R>, String>,
{
    f: F,
}

impl<F, R> Parser<R> for P2<F, R>
where
    F: Fn(String) -> Result<Box<R>, String>,
{
    fn parse(&self, text: String) -> Result<Box<R>, String> {
        (self.f)(text)
    }
}

fn into_parser<F, R>(f: F) -> Box<dyn Parser<R>>
where
    F: Fn(&str) -> Result<Box<R>, String> + 'static,
    R: 'static,
{
    Box::new(P2 {
        f: move |text: String| f(&text),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let actual = Processor::<i32>::new()
            .parser(|_text: &str| Ok(Box::new(1)))
            .parse("test");
        assert_eq!(Ok(Box::new(1)), actual);

        let actual = Processor::<String>::new()
            .parser(|text: &str| Ok(Box::new(text.to_string())))
            .parse("test");
        assert_eq!(Ok(Box::new("test".to_string())), actual);

        let actual = Processor::<Vec<i32>>::new()
            .parser(|_: &str| Ok(Box::new(vec![1])))
            .parse("test");
        assert_eq!(Ok(Box::new(vec![1])), actual);

        let actual = Processor::<String>::new()
            .parser(|_: &str| -> Result<Box<String>, String> { Err("error".to_string()) })
            .parse("test");
        assert_eq!(Err("error".to_string()), actual);
    }
}
