fn signed_integer() -> String {
    let numerik1 = 8;
    let numerik2: i8 = 12;
    let numerik3: i64 = 1000;

    String::from(std::format!("{} {} {}", numerik1, numerik2, numerik3))
}

fn floating_point() -> String {
    let fp1: f32 = 3.14;
    let fp2: f64 = 3.1415926535;

    String::from(std::format!("{} {}", fp1, fp2))
}

fn char() -> String {
    let c1 = '1';
    let c2 = '-';
    let c3 = '2';

    String::from(std::format!("{} {} {}", c1, c2, c3))
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::signed_integer;
    use super::floating_point;
    use super::char;

    #[test]
    fn test_signed_integer() {
        assert_eq!(signed_integer(), "8 12 1000")
    }

    #[test]
    fn test_floating_point() {
        assert_eq!(floating_point(), "3.14 3.1415926535")
    }

    #[test]
    fn test_char() {
        assert_eq!(char(), "1 - 2")
    }

    #[test]
    fn test_pointer() {
        let ptr1: &i32 = &64;

        assert_eq!(ptr1, &64)
    }

    #[test]
    fn test_string_escape() {
        let var = "hello \
        \"rust\" \
        and \
        \"world\"";

        assert_eq!(var, "hello \"rust\" and \"world\"")
    }

    #[test]
    fn test_raw_string() {
        let var = r#"
            {
                "name": "Issac",
                "gender": "Male"
            }
        "#;

        let result = r#"
            {
                "name": "Issac",
                "gender": "Male"
            }
        "#;

        assert_eq!(var, result)
    }

    #[test]
    fn test_constant() {
        const LABEL: &str = "nilai pi adalah:";
        const PI: f32 = 22.0/7.0;

        let result = String::from(std::format!("{} {}", LABEL, PI));

        assert_eq!(result, std::format!("{} {}", LABEL, PI))
    }
}
