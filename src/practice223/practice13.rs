fn count_permutation(shipments: &[u32]) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len();

    if total % n as u32 != 0 {
        return None;
    }

    let target = total / n as u32;
    let mut balance = 0;
    let mut moves = 0;

    for &cargo in shipments {
        balance += (cargo as i32) - (target as i32);
        moves += balance.abs() as usize;
    }

    Some(moves)
}

fn gen_shipments(n: usize, avg_weight: u32) -> Vec<u32> {
    let mut shipments = vec![avg_weight; n];


    let extra_ships = 0;

    for i in 0..(extra_ships % n) {
        shipments[i] += 1;
    }

    shipments
}

fn main() {
    // Приклад 1
    let shipments1 = vec![8, 2, 2, 4, 4];
    println!("Вантажі: {:?}", shipments1);
    match count_permutation(&shipments1) {
        Some(moves) => println!("Мінімальні переміщення для рівного розподілу: {}", moves),
        None => println!("Неможливо розподілити вантаж рівномірно."),
    }

    // Приклад 2
    let shipments2 = vec![9, 3, 7, 2, 9];
    println!("Вантажі: {:?}", shipments2);
    match count_permutation(&shipments2) {
        Some(moves) => println!("Мінімальні переміщення для рівного розподілу: {}", moves),
        None => println!("Неможливо розподілити вантаж рівномірно."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_permutation() {
        assert_eq!(count_permutation(&vec![8, 2, 2, 4, 4]), Some(4));
        assert_eq!(count_permutation(&vec![9, 3, 7, 2, 9]), Some(7));
        assert_eq!(count_permutation(&vec![10, 20, 30]), Some(20));
        assert_eq!(count_permutation(&vec![10, 10, 10]), Some(0));
    }

    #[test]
    fn test_gen_shipments() {
        let shipments = gen_shipments(5, 4);
        assert_eq!(shipments, vec![4, 4, 4, 4, 4]);
    }
}
