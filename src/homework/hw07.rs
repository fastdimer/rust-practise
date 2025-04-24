pub fn invert_the_case(s: String) -> String {
    let mut result = String::new();

    for c in s.chars() {
        if c.is_uppercase() {
            result.push_str(&c.to_lowercase().to_string());
        } else if c.is_lowercase() {
            result.push_str(&c.to_uppercase().to_string());
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invert_case() {
        let data = [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];

        for (a, b) in data {
            let result = invert_the_case(a.to_string());
            println!("{} => {}", a, result);
            assert_eq!(result, b.to_string());

            let result_back = invert_the_case(b.to_string());
            println!("{} => {}", b, result_back);
            assert_eq!(result_back, a.to_string());
        }
    }
}