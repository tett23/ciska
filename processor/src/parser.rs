pub trait Parser<R> {
    fn parse(&self, text: String) -> Result<R, String>;
}

impl<F, R> From<F> for Box<dyn Parser<R>>
where
    F: Fn(&str) -> Result<R, String> + 'static,
    R: 'static,
{
    fn from(f: F) -> Self {
        into_parser(f)
    }
}

struct P2<F, R>
where
    F: Fn(String) -> Result<R, String>,
{
    f: F,
}

impl<F, R> Parser<R> for P2<F, R>
where
    F: Fn(String) -> Result<R, String>,
{
    fn parse(&self, text: String) -> Result<R, String> {
        (self.f)(text)
    }
}

fn into_parser<F, R>(f: F) -> Box<dyn Parser<R>>
where
    F: Fn(&str) -> Result<R, String> + 'static,
    R: 'static,
{
    Box::new(P2 {
        f: move |text: String| f(&text),
    })
}

pub struct ParserProcessor<R> {
    parser: Box<dyn Parser<R>>,
}

impl<R> ParserProcessor<R> {
    pub fn new(f: Box<dyn Parser<R>>) -> ParserProcessor<R> {
        ParserProcessor { parser: f }
    }

    pub fn parse(&self, text: &str) -> Result<R, String> {
        self.parser.parse(text.to_string())
    }
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
