// #![feature(inherent_associated_types)]

mod parser;

use parser::ParserProcessor;

// pub enum Processor<'a, N> {
//     None,
//     ParserProcessor(ParserProcessor<'a, N>),
// }

// impl<'a, N> Processor<'a, N> {
//     pub fn new() -> Processor<'a, N> {
//         Processor::None
//     }

//     pub fn parser<F>(&self, f: F) -> Processor<N>
//     where
//         F: Fn(&str) -> Result<N, String>,
//     {
//         ParserProcessor::new(ParserProcessor::new(f))
//     }

//     pub fn parse(&self, text: &str) -> Result<String, String> {
//         match self {
//             Processor::None => Err("".to_string()),
//             Processor::ParserProcessor(processor) => Ok("".to_string()),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let actual = Processor::<String>::new()
        //     .parser(|text: String| Err("".to_string()))
        //     .parse("");
        // assert_eq!(Err("".to_string()), actual);
    }
}
