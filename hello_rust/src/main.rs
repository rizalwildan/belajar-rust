fn greet() -> String {
    String::from("Hello, World!")
}

fn main() {
    println!("{}", greet());
}

#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    fn test_hello() {
        assert_eq!(greet(), "Hello, World!");
    }
}