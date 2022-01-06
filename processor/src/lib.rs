mod formatter;
mod parser;
mod transformer;

use formatter::Formatter;
use parser::Parser;
use transformer::Transformer;

pub struct Processor<T, R>
where
    T: 'static,
    R: 'static,
{
    parser: Option<Box<dyn Parser<T>>>,
    transformer: Option<Box<dyn Transformer<T, T>>>,
    formatter: Option<Box<dyn Formatter<T, R>>>,
}

impl<T, R> Processor<T, R> {
    pub fn new() -> Processor<T, R> {
        Processor {
            parser: None,
            transformer: None,
            formatter: None,
        }
    }

    pub fn parser<F>(&self, f: F) -> Processor<T, R>
    where
        F: Fn(&str) -> Result<T, String> + 'static + Clone,
        T: Clone,
        R: Clone,
    {
        Processor {
            parser: Some(f.into()),
            transformer: match &self.transformer {
                Some(v) => Some(dyn_clone::clone_box(&**v)),
                None => None,
            },
            formatter: match &self.formatter {
                Some(v) => Some(dyn_clone::clone_box(&**v)),
                None => None,
            },
        }
    }

    pub fn parse(&self, text: &str) -> Result<T, String> {
        match self {
            Processor {
                parser: Some(parser),
                ..
            } => parser.parse(text),
            _ => Err("".to_string()),
        }
    }

    pub fn formatter<F>(&self, f: F) -> Processor<T, R>
    where
        F: Fn(&T) -> Result<R, String> + 'static + Clone,
        T: Clone,
        R: Clone,
    {
        Processor {
            parser: match &self.parser {
                Some(v) => Some(dyn_clone::clone_box(&**v)),
                None => None,
            },
            transformer: match &self.transformer {
                Some(v) => Some(dyn_clone::clone_box(&**v)),
                None => None,
            },
            formatter: Some(f.into()),
        }
    }

    pub fn format(&self, ast: &T) -> Result<R, String> {
        match self {
            Processor {
                formatter: Some(formatter),
                ..
            } => formatter.format(ast),
            _ => Err("".to_string()),
        }
    }

    pub fn transformer<F>(&self, f: F) -> Processor<T, R>
    where
        F: Fn(&T) -> Result<T, String> + 'static + Clone,
        T: Clone,
        R: Clone,
    {
        Processor {
            parser: match &self.parser {
                Some(v) => Some(dyn_clone::clone_box(&**v)),
                None => None,
            },
            transformer: match &self.transformer {
                Some(v) => Some(dyn_clone::clone_box(&**v)),
                None => Some(f.into()),
            },
            formatter: match &self.formatter {
                Some(v) => Some(dyn_clone::clone_box(&**v)),
                None => None,
            },
        }
    }

    pub fn transform(&self, ast: &T) -> Result<T, String> {
        match self {
            Processor {
                transformer: Some(transformer),
                ..
            } => transformer.transform(ast),
            _ => Err("".to_string()),
        }
    }

    pub fn process(&self, text: &str) -> Result<R, String> {
        let ast = self.parse(text)?;
        let ast = match &self.transformer {
            Some(transformer) => transformer.transform(&ast),
            None => Ok(ast),
        };
        self.format(&ast?)
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
            .formatter(|_text: &i32| Ok(1))
            .process("test");
        assert_eq!(Ok(1), actual);

        let actual = Processor::<String, String>::new()
            .parser(|text: &str| Ok(text.to_string()))
            .formatter(|text: &String| Ok(text.clone()))
            .process("test");
        assert_eq!(Ok("test".to_string()), actual);

        let actual = Processor::<Vec<i32>, Vec<i32>>::new()
            .parser(|_: &str| Ok(vec![1]))
            .formatter(|_text| Ok(vec![1]))
            .parse("test");
        assert_eq!(Ok(vec![1]), actual);

        let actual = Processor::<String, String>::new()
            .parser(|_: &str| Err("error".to_string()))
            .formatter(|_text| Err("error".to_string()))
            .parse("test");
        assert_eq!(Err("error".to_string()), actual);
    }
}
