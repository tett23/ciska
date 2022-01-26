use dyn_clone::{clone_box, DynClone};
use std::marker::PhantomData;

pub trait Transformer<T, U>: DynClone
where
    T: 'static,
    U: 'static,
{
    fn transform(&self, ast: &T) -> Result<U, String>;
}

impl<T, U> Clone for Box<dyn Transformer<T, U>> {
    fn clone(&self) -> Self {
        clone_box(&**self)
    }
}

impl<F, T, U> From<F> for Box<dyn Transformer<T, U>>
where
    F: Fn(&T) -> Result<U, String> + 'static + Clone,
    T: 'static + Clone,
    U: 'static + Clone,
{
    fn from(f: F) -> Self {
        into_transformer(f)
    }
}

#[derive(Clone)]
struct TransformProcessor<F, T, U>
where
    F: Fn(&T) -> Result<U, String> + 'static,
    T: 'static,
    U: 'static,
{
    f: F,
    _t: PhantomData<T>,
}

impl<F, T, U> Transformer<T, U> for TransformProcessor<F, T, U>
where
    F: Fn(&T) -> Result<U, String> + 'static + Clone,
    T: 'static + Clone,
    U: 'static + Clone,
{
    fn transform(&self, ast: &T) -> Result<U, String> {
        (self.f)(ast)
    }
}

fn into_transformer<F, T, U>(f: F) -> Box<dyn Transformer<T, U>>
where
    F: Fn(&T) -> Result<U, String> + 'static + Clone,
    T: 'static + Clone,
    U: 'static + Clone,
{
    let f = move |ast: &T| f(ast);
    let r = Box::new(TransformProcessor { f, _t: PhantomData });

    r
}

pub(crate) fn compose_transformer<A>(
    f: Box<dyn Transformer<A, A>>,
    g: Box<dyn Transformer<A, A>>,
) -> Box<dyn Transformer<A, A>>
where
    // F: 'static + Clone + Transformer<A, A>,
    // G: 'static + Clone + Transformer<A, A>,
    A: 'static + Clone,
{
    into_transformer(move |a: &A| f.transform(a).map(|b| g.transform(&b).unwrap()))
}
