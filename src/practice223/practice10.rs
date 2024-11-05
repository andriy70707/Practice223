fn rotate(s: String, n: isize) -> String {
    let len = s.len();

    // Повертаємо нульові зсуви
    if len == 0 || n == 0 || n.abs() as usize % len == 0 {
        return s;
    }

    // Знайдемо фактичне значення зсуву
    let n = ((n % len as isize) + len as isize) % len as isize;

    // Виконаємо обертання
    let split_index = len - n as usize;
    let (left, right) = s.split_at(split_index);
    format!("{}{}", right, left)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh".to_string();
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

        shifts.iter().for_each(|(n, exp)| {
            assert_eq!(rotate(s.clone(), *n), exp.to_string());
        });
    }
}
