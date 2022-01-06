// use std::rc::Rc;

pub trait Parser<R> {
    fn parse(&self, text: String) -> Result<Box<R>, String>;
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

pub struct ParserProcessor<R>
where
// R: 'a,
{
    parser: Box<dyn Parser<R>>,
}

impl<R> ParserProcessor<R>
where
// R: 'a,
{
    pub fn new(f: Box<dyn Parser<R>>) -> ParserProcessor<R> {
        // let a: Box<dyn Parser<R> + 'a> = Box::new(P2 { f });
        // let a: Box<dyn Parser<R>> = Box::new(P2 { f });

        ParserProcessor { parser: f }
    }

    pub fn parse(&self, text: &str) -> Result<Box<R>, String> {
        self.parser.parse(text.to_string())
    }

    // fn new(f: impl Fn(&str) -> R + 'a) -> &dyn P2<R> {}

    // pub fn parser<F, R>(&self, parser: Box<F>) -> Processor<F, R>
    // where
    //     F: Fn(&str) -> R,
    // {
    //     let f = Processor::new.clone();
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let actual = ParserProcessor::<String>::new(Box::new(P2 {
            f: |a: String| Err(a.to_string()),
        }));
        let actual = actual.parser.parse("".to_string());

        assert_eq!(Err("".to_string()), actual);
    }
}
