const N: usize = 10;
#[test]
fn main() {
    let mut romб = String::new();

    for i in 0..N {
        let пробіли = N - i - 1;
        let зірочки = 2 * i + 1;

        // Додаємо пробіли
        for _ in 0..пробіли {
            romб.push(' ');
        }

        for _ in 0..зірочки {
            romб.push('*');
        }

        romб.push('\n');
    }

    for i in (0..N - 1).rev() {
        let пробіли = N - i - 1;
        let зірочки = 2 * i + 1;

        for _ in 0..пробіли {
            romб.push(' ');
        }

        for _ in 0..зірочки {
            romб.push('*');
        }

        romб.push('\n');
    }

    print!("{}", romб);
}
