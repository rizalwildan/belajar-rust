fn comparison(value1: i32, value2: i32) -> String {
    if value1 == value2 {
        String::from("equal")
    } else if value1 > value2 {
        String::from("greater then")
    } else if value1 < value2 {
        String::from("smaller then")
    } else if value1 >= value2 {
        String::from("greater or equal")
    } else {
        String::from("smaller or equal")
    }
}

fn main() {
    println!("Hello, operator!");
}

#[cfg(test)]
mod tests {
    use super::comparison;

    #[test]
    fn test_sum() {
        let value1 = 10;
        let value2 = 20;

        let result = value1 + value2;

        assert_eq!(result, 30)
    }

    #[test]
    fn test_subtraction() {
        let value1 = 10;
        let value2 = 9;

        let result = value1 - value2;

        assert_eq!(result, 1);
    }

    #[test]
    fn test_multiply() {
        let value1 = 4;
        let value2 = 2;

        let result = value1 * value2;

        assert_eq!(result, 8);
    }

    #[test]
    fn test_div() {
        let value1 = 8;
        let value2 = 4;

        let result = value1 / value2;

        assert_eq!(result, 2);
    }

    #[test]
    fn test_mod() {
        let value1 = 12;
        let value2 = 4;

        let result = value1 % value2;

        assert_eq!(result, 0);
    }

    #[test]
    fn test_comparison_equal() {
        let value1 = 12;
        let value2 = 12;

        assert_eq!(comparison(value1, value2), "equal")
    }

    #[test]
    fn test_comparison_greater() {
        let value1 = 3;
        let value2 = 1;

        assert_eq!(comparison(value1, value2), "greater then")
    }

    #[test]
    fn test_comparison_smaller() {
        let value1 = 3;
        let value2 = 10;

        assert_eq!(comparison(value1, value2), "smaller then")
    }
}
