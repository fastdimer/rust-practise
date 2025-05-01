fn rotate(s: &str, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s.to_string();
    }

    let n = ((n % len as isize) + len as isize) % len as isize;
    let n = n as usize;

    format!("{}{}", &s[len - n..], &s[..len - n])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh";
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        for (n, exp) in shifts {
            let result = rotate(s, n);
            println!("rotate({}, {}) = {}", s, n, result);
            assert_eq!(rotate(s, n), exp.to_string());
        }
    }
}