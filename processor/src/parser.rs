use dyn_clone::DynClone;

pub trait Parser<R>: DynClone {
    fn parse(&self, text: &str) -> Result<R, String>;
}

impl<F, R> From<F> for Box<dyn Parser<R>>
where
    F: Fn(&str) -> Result<R, String> + 'static + Clone,
    R: 'static + Clone,
{
    fn from(f: F) -> Self {
        into_parser(f)
    }
}

#[derive(Clone)]
struct ParserProcessor<F, R>
where
    F: Fn(&str) -> Result<R, String> + Clone,
    R: Clone,
{
    f: F,
}

impl<F, R> Parser<R> for ParserProcessor<F, R>
where
    F: Fn(&str) -> Result<R, String> + Clone,
    R: Clone,
{
    fn parse(&self, text: &str) -> Result<R, String> {
        (self.f)(text)
    }
}

fn into_parser<F, R>(f: F) -> Box<dyn Parser<R>>
where
    F: Fn(&str) -> Result<R, String> + 'static + Clone,
    R: 'static + Clone,
{
    Box::new(ParserProcessor {
        f: move |text: &str| f(&text),
    })
}
