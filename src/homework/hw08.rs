fn is_prime(n: &u32) -> bool {
    if *n <= 1 {
        return false;
    }

    let limit = (*n as f64).sqrt() as u32;
    for i in 2..=limit {
        if *n % i == 0 {
            return false;
        }
    }

    true
}

#[test]
fn test_is_prime() {
    let test_data = [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (100, false),
        (10007, true),
    ];

    for (n, prime) in test_data {
        assert_eq!(is_prime(&n), prime);
    }
    println!("всі тести вірні");
}