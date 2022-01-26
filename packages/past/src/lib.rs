trait Node {
    fn name(&self) -> String;
}

trait Valuable<T> {
    fn value(&self) -> V;
}

trait Parentable<T> {
    fn children(&self) -> Vec<T>;
}

trait Datable<T> {
    fn data(&self) -> T;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
