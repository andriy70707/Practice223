#[test]
fn main() {
    let levels = 6;
    draw_tree(levels);
}

fn draw_tree(levels: usize) {
    let mut output = String::new();

    (0..levels).for_each(|i| {
        (0..=i + 1).for_each(|j| {

            let spaces = " ".repeat(levels - j);
            let stars = "*".repeat(2 * j + 1);
            output.push_str(&format!("{}{}\n", spaces, stars));
        });
    });

    (0..(levels / 4).max(1).min(3)).for_each(|_| {
        let trunk_spaces = " ".repeat(levels);
        output.push_str(&format!("{}||\n", trunk_spaces));
    });

    println!("{}", output);
}
