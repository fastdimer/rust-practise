use std::collections::HashMap;

fn gray(n: u8) -> Vec<String> {
    fn helper(n: u8, cache: &mut HashMap<u8, Vec<String>>) -> Vec<String> {
        if let Some(cached) = cache.get(&n) {
            return cached.clone();
        }

        let result = match n {
            0 => vec![String::from("")],
            1 => vec![String::from("0"), String::from("1")],
            _ => {
                let prev = helper(n - 1, cache);
                let mut res = Vec::new();
                res.extend(prev.iter().map(|s| format!("0{}", s)));
                res.extend(prev.iter().rev().map(|s| format!("1{}", s)));
                res
            }
        };

        cache.insert(n, result.clone());
        result
    }

    helper(n, &mut HashMap::new())
}

fn main() {
    let result = gray(3);
    for s in result {
        println!("{}", s);
    }
}