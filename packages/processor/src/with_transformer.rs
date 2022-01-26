use crate::formatter::Formatter;
use crate::parser::Parser;
use crate::processable::{Formattable, Parsable, Processable, Transformable};
use crate::transformer::Transformer;

// #[derive(Clone)]
pub struct WithTransformer<T, R>
where
    T: 'static + Clone,
    R: 'static,
{
    pub parser: Option<Box<dyn Parser<T>>>,
    pub transformer: Option<Box<dyn Transformer<T, T>>>,
    pub formatter: Option<Box<dyn Formatter<T, R>>>,
}

impl<T: Clone, R> Clone for WithTransformer<T, R> {
    fn clone(&self) -> Self {
        WithTransformer {
            parser: self.parser.clone(),
            transformer: self.transformer.clone(),
            formatter: self.formatter.clone(),
        }
    }
}

impl<T: Clone, R> WithTransformer<T, R> {
    pub fn new() -> Self {
        Self {
            parser: None,
            transformer: None,
            formatter: None,
        }
    }
}

impl<T: Clone, R> Parsable<T> for WithTransformer<T, R> {
    fn parse(&self, text: &str) -> Result<T, String> {
        match self {
            WithTransformer {
                parser: Some(parser),
                ..
            } => parser.parse(text),
            _ => Err("".to_string()),
        }
    }
}

impl<T: Clone, R> Transformable<T> for WithTransformer<T, R> {
    fn transform(&self, ast: &T) -> Result<T, String> {
        match self {
            WithTransformer {
                transformer: Some(transformer),
                ..
            } => transformer.transform(ast),
            _ => Err("".to_string()),
        }
    }
}

impl<T: Clone, R> Formattable<T, R> for WithTransformer<T, R> {
    fn format(&self, ast: &T) -> Result<R, String> {
        match self {
            WithTransformer {
                formatter: Some(formatter),
                ..
            } => formatter.format(ast),
            _ => Err("".to_string()),
        }
    }
}

impl<T: Clone, R> Processable<T, R> for WithTransformer<T, R> {}
