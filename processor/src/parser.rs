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

struct ParserProcessor<F, R>
where
    F: Fn(String) -> Result<R, String>,
{
    f: F,
}

impl<F, R> Parser<R> for ParserProcessor<F, R>
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
    Box::new(ParserProcessor {
        f: move |text: String| f(&text),
    })
}
