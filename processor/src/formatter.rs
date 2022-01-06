use dyn_clone::DynClone;
use std::marker::PhantomData;

pub trait Formatter<T, R>: DynClone
where
    T: 'static,
    R: 'static,
{
    fn format(&self, ast: &T) -> Result<R, String>;
}

impl<F, T, R> From<F> for Box<dyn Formatter<T, R>>
where
    F: Fn(&T) -> Result<R, String> + 'static + Clone,
    T: 'static + Clone,
    R: 'static + Clone,
{
    fn from(f: F) -> Self {
        into_parser(f)
    }
}

#[derive(Clone)]
struct FormatterProcessor<F, T, R>
where
    F: Fn(&T) -> Result<R, String> + 'static,
    T: 'static,
    R: 'static,
{
    f: F,
    _t: PhantomData<T>,
}

impl<F, T, R> Formatter<T, R> for FormatterProcessor<F, T, R>
where
    F: Fn(&T) -> Result<R, String> + 'static + Clone,
    T: 'static + Clone,
    R: 'static + Clone,
{
    fn format(&self, ast: &T) -> Result<R, String> {
        (self.f)(ast)
    }
}

fn into_parser<F, T, R>(f: F) -> Box<(dyn Formatter<T, R>)>
where
    F: Fn(&T) -> Result<R, String> + 'static + Clone,
    T: 'static + Clone,
    R: 'static + Clone,
{
    let f = move |ast: &T| f(ast);
    let r = Box::new(FormatterProcessor { f, _t: PhantomData });

    r
}
