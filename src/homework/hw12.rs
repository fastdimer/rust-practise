use rand::Rng;

fn calculate_balance_steps(shipments: &Vec<u32>) -> isize {
    let total: u32 = shipments.iter().sum();
    let count = shipments.len();

    if total % count as u32 != 0 {
        return isize::MAX;
    }

    let average = total / count as u32;
    let mut moves = 0;
    let mut imbalance = 0;

    for &shipment in shipments {
        imbalance += shipment as i32 - average as i32;
        moves += imbalance.abs() as usize;
    }

    moves as isize
}

fn generate_balanced_vector(size: usize) -> Vec<u32> {
    let average_value = rand::thread_rng().gen_range(1..100);
    let mut vector = vec![average_value; size];

    for i in 0..size / 2 {
        let variation = rand::thread_rng().gen_range(0..=average_value.min(10));
        vector[i] += variation;
        vector[size - 1 - i] -= variation;
    }

    vector
}

pub fn main() {
    let example_one = vec![8, 2, 2, 4, 4];
    let result_one = calculate_balance_steps(&example_one);
    println!("Example 1: {:?} => Moves: {}", example_one, result_one);

    let example_two = vec![9, 3, 7, 2, 9];
    let result_two = calculate_balance_steps(&example_two);
    println!("Example 2: {:?} => Moves: {}", example_two, result_two);

    let example_three = vec![1, 1, 1, 1, 6];
    let result_three = calculate_balance_steps(&example_three);
    println!("Example 3: {:?} => Moves: {}", example_three, result_three);

    let generated = generate_balanced_vector(10);
    println!("Generated vector: {:?}", generated);
    let generated_result = calculate_balance_steps(&generated);
    println!("Moves required to balance generated vector: {}", generated_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_one() {
        let test_data = vec![8, 2, 2, 4, 4];
        assert_eq!(calculate_balance_steps(&test_data), 4);
    }

    #[test]
    fn test_case_two() {
        let test_data = vec![9, 3, 7, 2, 9];
        assert_eq!(calculate_balance_steps(&test_data), 7);
    }

    #[test]
    fn test_impossible_balance() {
        let test_data = vec![1, 2, 3];
        assert_eq!(calculate_balance_steps(&test_data), isize::MAX);
    }
}