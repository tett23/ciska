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

    pub fn parser(&self, f: Box<dyn Parser<N>>) -> Processor<N> {
        Processor::ParserProcessor(ParserProcessor::new(f))
    }

    pub fn parse(&self, text: &str) -> Result<Box<N>, String> {
        match self {
            Processor::None => Err("".to_string()),
            Processor::ParserProcessor(processor) => processor.parse(text),
        }
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

// impl<R> From<Box<dyn Fn(&str) -> R>> for Box<dyn Parser<R>> {
//     fn from(f: Box<dyn Fn(&str) -> R>) -> Self {
//         Box::new(P2 {
//             f: |a: String| Box::new(Err(a.to_string())),
//         })
//     }
// }

fn into_parser<F, R>(f: F) -> Box<dyn Parser<R>>
where
    F: Fn(&str) -> Result<Box<R>, String> + 'static,
    R: 'static,
{
    let c = move |a: String| {
        let b = match f(&a) {
            Ok(v) => Result::Ok(v),
            Err(e) => Err(e),
        };

        b
    };

    Box::new(P2 { f: c })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let actual = Processor::<String>::new()
            // .parser(|text: &str| Err(Box::new("".to_string())))
            .parser(into_parser(|text: &str| Ok(Box::new(text.to_string()))))
            .parse("test");
        assert_eq!(Ok(Box::new("test".to_string())), actual);
    }
}
