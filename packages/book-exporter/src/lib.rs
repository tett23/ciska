#![feature(generators, generator_trait)]

#[macro_use]
extern crate diesel;

mod schemas;
mod use_cases;

use use_cases::books;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        dbg!(books());

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
