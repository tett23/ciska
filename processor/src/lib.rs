mod parser;

use parser::ParserProcessor;

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
        F: Fn(&str) -> Result<N, String> + 'static,
    {
        Processor::ParserProcessor(ParserProcessor::new(f.into()))
    }

    pub fn parse(&self, text: &str) -> Result<N, String> {
        match self {
            Processor::None => Err("".to_string()),
            Processor::ParserProcessor(processor) => processor.parse(text),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let actual = Processor::<i32>::new()
            .parser(|_text: &str| Ok(1))
            .parse("test");
        assert_eq!(Ok(1), actual);

        let actual = Processor::<String>::new()
            .parser(|text: &str| Ok(text.to_string()))
            .parse("test");
        assert_eq!(Ok("test".to_string()), actual);

        let actual = Processor::<Vec<i32>>::new()
            .parser(|_: &str| Ok(vec![1]))
            .parse("test");
        assert_eq!(Ok(vec![1]), actual);

        let actual = Processor::<String>::new()
            .parser(|_: &str| Err("error".to_string()))
            .parse("test");
        assert_eq!(Err("error".to_string()), actual);
    }
}
