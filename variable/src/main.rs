fn variable() -> String {
    let name_variable = "predefined variable";

    String::from(name_variable)
}

fn immutable_variable(number: i32) -> String {
    let mut message_number = 1;
    let message1 = "hello";

    if number != 0 {
        message_number = number
    }

    String::from(std::format!("message number: {} | {}", message_number, message1))
}

fn shadowing_variable(multiply: i32) -> i32 {
    let x = 5;

    x * multiply
}

fn main() {
    println!("{}", variable());
    println!("{}", immutable_variable(0));
    println!("{}", immutable_variable(2));
    println!("multiply result: {}", shadowing_variable(5));
}

#[cfg(test)]
mod tests {
    use super::variable;
    use super::immutable_variable;
    use super::shadowing_variable;

    #[test]
    fn test_variable() {
        assert_eq!(variable(), "predefined variable");
    }

    #[test]
    fn test_immutable_variable() {
        let number = 0;
        assert_eq!(immutable_variable(number), "message number: 1 | hello")
    }

    #[test]
    fn test_immutable_variable_with_added_number() {
        let number = 2;
        assert_eq!(immutable_variable(number), "message number: 2 | hello")
    }

    #[test]
    fn test_shadowing_variable() {
        let multiply = 5;
        assert_eq!(shadowing_variable(multiply), 25)
    }
}
