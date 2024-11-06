use std::cmp::{max, min};

struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    fn area(&self) -> i32 {
        let width = (self.b.x - self.a.x).abs();
        let height = (self.b.y - self.a.y).abs();
        width * height
    }

    fn overlap_area(&self, other: &Rectangle) -> i32 {
        let x_overlap = max(0, min(self.b.x, other.b.x) - max(self.a.x, other.a.x));
        let y_overlap = max(0, min(self.b.y, other.b.y) - max(self.a.y, other.a.y));
        x_overlap * y_overlap
    }
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut total_area = 0;
    for i in 0..xs.len() {
        total_area += xs[i].area();
    }

    let mut overlap_area = 0;
    for i in 0..xs.len() {
        for j in i + 1..xs.len() {
            overlap_area += xs[i].overlap_area(&xs[j]);
        }
    }

    total_area - overlap_area
}

fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
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

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle { a: Point { x: 2, y: 9 }, b: Point { x: 5, y: 3 } },
        Rectangle { a: Point { x: 1, y: 8 }, b: Point { x: 11, y: 6 } },
        Rectangle { a: Point { x: 9, y: 10 }, b: Point { x: 13, y: 2 } },
    ]
}

fn shipment_data() -> Vec<u32> {
    vec![8, 2, 2, 4, 4]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_occupied_test() {
        let data = test_data();
        let occupied = area_occupied(&data);
        assert_eq!(occupied, 60);
    }

    #[test]
    fn count_permutation_test() {
        let data = shipment_data();
        let moves = count_permutation(&data);
        assert_eq!(moves, Some(4));
    }
}

fn main() {
    let data = test_data();
    let occupied = area_occupied(&data);
    println!("Фактична зайнята площа: {}", occupied);

    let shipments = shipment_data();
    let moves = count_permutation(&shipments);
    match moves {
        Some(m) => println!("Мінімальна кількість переміщень: {}", m),
        None => println!("Неможливо рівно розподілити вантажі"),
    }
}
