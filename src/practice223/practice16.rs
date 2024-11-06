use itertools::Itertools;

fn solve() {
    let mut count = 0;

    // Визначаємо числа від 1 до 8
    let digits = vec![1, 2, 3, 4, 5, 6, 7, 8];

    // Перебираємо всі можливі перестановки чисел для букв
    for perm in digits.iter().permutations(8) {
        let m = perm[0];
        let u = perm[1];
        let x = perm[2];
        let a = perm[3];
        let s = perm[4];
        let l = perm[5];
        let o = perm[6];
        let n = perm[7];

        // Обчислюємо значення для muxa і slon
        let muxa = m * 1000 + u * 100 + x * 10 + a;
        let slon = s * 1000 + l * 100 + o * 10 + n;
        let x_val = x;

        // Перевіряємо умову: muxa == x * a + slon
        if muxa == x_val * a + slon {
            count += 1;

            // Виведення результату
            println!("  {}{}{}{}", m, u, x, a);
            println!("{}        {}", x, a);
            println!("  ------");
            println!("    {}{}{}{}", s, l, o, n);
        }
    }

    // Виведення кількості рішень
    println!("Кількість рішень: {}", count);
}

fn main() {
    solve();
}
