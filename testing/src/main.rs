fn caps(input: &str) -> String {
    input.to_uppercase()
}
fn main() {}
#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn check() {
        let result = caps("hello world!");
        let expected = String::from("HELLO WORLD!");
        assert_eq!(result, expected, "string should be all uppercase");
    }
}
