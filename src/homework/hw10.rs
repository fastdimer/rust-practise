fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    let reversed: String = s.chars().rev().collect();
    s == reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_test() {
        let data = [
            (123, false),
            (121, true),
            (1221, true),
        ];
        data.iter().for_each(|(n, exp)| {
            let result = is_palindrome(*n);
            println!("is_palindrome({}) = {}, expected = {}", n, result, exp);
            assert_eq!(is_palindrome(*n), *exp);
        });
    }
}