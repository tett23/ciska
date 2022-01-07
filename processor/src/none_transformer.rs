use crate::formatter::Formatter;
use crate::parser::Parser;
use crate::processable::{Formattable, Parsable, Processable, Transformable};

#[derive(Clone)]
pub struct NoneTransformer<T, R>
where
    T: 'static + Clone,
    R: 'static,
{
    pub parser: Option<Box<dyn Parser<T>>>,
    pub formatter: Option<Box<dyn Formatter<T, R>>>,
}

impl<T: Clone, R> NoneTransformer<T, R> {
    pub fn new() -> Self {
        Self {
            parser: None,
            formatter: None,
        }
    }
}

impl<T: Clone, R> Parsable<T> for NoneTransformer<T, R> {
    fn parse(&self, text: &str) -> Result<T, String> {
        match self {
            NoneTransformer {
                parser: Some(parser),
                ..
            } => parser.parse(text),
            _ => Err("".to_string()),
        }
    }
}

impl<T: Clone, R> Transformable<T> for NoneTransformer<T, R> {
    fn transform(&self, ast: &T) -> Result<T, String> {
        Ok(ast.clone())
    }
}

impl<T: Clone, R> Formattable<T, R> for NoneTransformer<T, R> {
    fn format(&self, ast: &T) -> Result<R, String> {
        match self {
            NoneTransformer {
                formatter: Some(formatter),
                ..
            } => formatter.format(ast),
            _ => Err("".to_string()),
        }
    }
}

impl<T: Clone, R> Processable<T, R> for NoneTransformer<T, R> where T: Clone {}
