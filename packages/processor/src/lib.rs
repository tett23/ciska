mod formatter;
mod none_transformer;
mod parser;
mod processable;
mod transformer;
mod with_transformer;

use none_transformer::NoneTransformer;
use processable::{Formattable, Parsable, Processable, Transformable};
use transformer::compose_transformer;
use with_transformer::WithTransformer;

pub enum Processor<T, R>
where
    T: 'static + Clone,
    R: 'static,
{
    NoneTransformer(NoneTransformer<T, R>),
    WithTransformer(WithTransformer<T, R>),
}

impl<T: Clone, R> Processor<T, R> {
    pub fn new() -> Processor<T, R> {
        Processor::NoneTransformer(NoneTransformer::new())
    }

    pub fn parser<F>(&self, f: F) -> Processor<T, R>
    where
        F: Fn(&str) -> Result<T, String> + 'static + Clone,
        T: Clone,
        R: Clone,
    {
        match self {
            Processor::NoneTransformer(a) => {
                let mut ret = a.clone();
                ret.parser = Some(f.into());
                Processor::NoneTransformer(ret)
            }
            Processor::WithTransformer(a) => {
                let mut ret = a.clone();
                ret.parser = Some(f.into());
                Processor::WithTransformer(ret)
            }
        }
    }

    pub fn formatter<F>(&self, f: F) -> Processor<T, R>
    where
        F: Fn(&T) -> Result<R, String> + 'static + Clone,
        T: Clone,
        R: Clone,
    {
        match self {
            Processor::NoneTransformer(a) => {
                let mut ret = a.clone();
                ret.formatter = Some(f.into());
                Processor::NoneTransformer(ret)
            }
            Processor::WithTransformer(a) => {
                let mut ret = a.clone();
                ret.formatter = Some(f.into());
                Processor::WithTransformer(ret)
            }
        }
    }

    pub fn transformer<F>(&self, f: F) -> Processor<T, R>
    where
        F: Fn(&T) -> Result<T, String> + 'static + Clone,
        T: Clone,
    {
        match self {
            Processor::NoneTransformer(a) => Processor::WithTransformer(WithTransformer {
                parser: a.parser.clone(),
                transformer: Some(f.into()),
                formatter: a.formatter.clone(),
            }),
            Processor::WithTransformer(a) => {
                let mut ret = a.clone();
                ret.transformer = Some(compose_transformer(
                    a.transformer.clone().unwrap(),
                    f.into(),
                ));
                Processor::WithTransformer(ret)
            }
        }
    }

    pub fn parse(&self, text: &str) -> Result<T, String> {
        match self {
            Processor::NoneTransformer(a) => a.parse(text),
            Processor::WithTransformer(a) => a.parse(text),
        }
    }

    pub fn transform(&self, ast: &T) -> Result<T, String>
    where
        T: Clone,
    {
        match self {
            Processor::NoneTransformer(a) => a.transform(ast),
            Processor::WithTransformer(a) => a.transform(ast),
        }
    }

    pub fn format(&self, ast: &T) -> Result<R, String> {
        match self {
            Processor::NoneTransformer(a) => a.format(ast),
            Processor::WithTransformer(a) => a.format(ast),
        }
    }

    pub fn process(&self, text: &str) -> Result<R, String> {
        match self {
            Processor::NoneTransformer(a) => a.process(text),
            Processor::WithTransformer(a) => a.process(text),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let actual = Processor::<i32, i32>::new()
            .parser(|_text: &str| Ok(1))
            .parse("test");
        assert_eq!(Ok(1), actual);

        let actual = Processor::<String, String>::new()
            .parser(|text: &str| Ok(text.to_string()))
            .parse("test");
        assert_eq!(Ok("test".to_string()), actual);

        let actual = Processor::<Vec<i32>, Vec<i32>>::new()
            .parser(|_: &str| Ok(vec![1]))
            .parse("test");
        assert_eq!(Ok(vec![1]), actual);

        let actual = Processor::<String, String>::new()
            .parser(|_: &str| Err("error".to_string()))
            .parse("test");
        assert_eq!(Err("error".to_string()), actual);
    }

    #[test]
    fn test_process() {
        let actual = Processor::<i32, i32>::new()
            .parser(|_text: &str| Ok(1))
            .transformer(|_text: &i32| Ok(1))
            .formatter(|_text: &i32| Ok(1))
            .process("test");
        assert_eq!(Ok(1), actual);

        let actual = Processor::<String, String>::new()
            .parser(|text: &str| Ok(text.to_string()))
            .transformer(|text: &String| Ok(text.clone()))
            .formatter(|text: &String| Ok(text.clone()))
            .process("test");
        assert_eq!(Ok("test".to_string()), actual);

        let actual = Processor::<Vec<i32>, Vec<i32>>::new()
            .parser(|_: &str| Ok(vec![1]))
            .transformer(|_text| Ok(vec![1]))
            .formatter(|_text| Ok(vec![1]))
            .parse("test");
        assert_eq!(Ok(vec![1]), actual);

        let actual = Processor::<String, String>::new()
            .parser(|_: &str| Err("error".to_string()))
            .transformer(|_text| Err("error".to_string()))
            .formatter(|_text| Err("error".to_string()))
            .parse("test");
        assert_eq!(Err("error".to_string()), actual);
    }
}
