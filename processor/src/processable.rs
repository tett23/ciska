pub(crate) trait Parsable<T: Clone> {
    fn parse(&self, text: &str) -> Result<T, String>;
}

pub(crate) trait Transformable<T: Clone> {
    fn transform(&self, ast: &T) -> Result<T, String>;
}
pub(crate) trait Formattable<T: Clone, R> {
    fn format(&self, ast: &T) -> Result<R, String>;
}

pub(crate) trait Processable<T: Clone, R>:
    Parsable<T> + Transformable<T> + Formattable<T, R>
{
    fn process(&self, text: &str) -> Result<R, String> {
        self.format(&self.transform(&self.parse(text)?)?)
    }
}
