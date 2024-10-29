#[test]
fn main() {
    const SIZE: usize = 18;
    let mut output = String::new();

    for y in 0..SIZE {
        for x in 0..SIZE {
            if y == 0 || y == SIZE - 1 {
                output.push('*');
            } else if x == 0 || x == SIZE - 1 {
                output.push('*');
            } else if x == y || x == (SIZE - y - 1) {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{}", output);
}
