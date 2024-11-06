use std::time::{SystemTime, UNIX_EPOCH};

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() + i as u128;
        let random_value = (seed % 90 + 10) as i32;
        result.push(random_value);
    }
    result
}

fn min_adjacent_sum(data: &[i32]) -> Option<i32> {
    if data.len() < 2 {
        return None;
    }
    data.windows(2).map(|w| w[0] + w[1]).min()
}

fn print_vector(data: &[i32]) {
    println!("Вектор: {:?}", data);
    data.windows(2).for_each(|w| {
        println!("Сусідні: ({}, {}), Сума: {}", w[0], w[1], w[0] + w[1]);
    });
}

fn main() {
    let random_vector = gen_random_vector(20);
    print_vector(&random_vector);

    if let Some(min_sum) = min_adjacent_sum(&random_vector) {
        println!("Мінімальна сума сусідніх елементів: {}", min_sum);
    } else {
        println!("Недостатньо елементів для обчислення мінімальної суми.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_random_vector() {
        let vec = gen_random_vector(20);
        assert_eq!(vec.len(), 20);
        assert!(vec.iter().all(|&x| x >= 10 && x < 100));
    }

    #[test]
    fn test_min_adjacent_sum() {
        let data = vec![12, 45, 32, 10, 99, 23];
        assert_eq!(min_adjacent_sum(&data), Some(42)); // 32 + 10 = 42 is the minimum adjacent sum
    }

    #[test]
    fn test_min_adjacent_sum_empty() {
        let data: Vec<i32> = vec![];
        assert_eq!(min_adjacent_sum(&data), None);
    }

    #[test]
    fn test_min_adjacent_sum_single_element() {
        let data = vec![10];
        assert_eq!(min_adjacent_sum(&data), None);
    }
}
