fn swap_case(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        //регістр верхній, змінюємо на нижній
        if c.is_uppercase() {
            result.push(c.to_lowercase().next().unwrap());
        //регістр нижній, змінюємо на верхній
        } else if c.is_lowercase() {
            result.push(c.to_uppercase().next().unwrap());

        } else {
            result.push(c);
        }
    }
    result
}

fn main() {
    let data = "Hello world!";
    let result = swap_case(data);

    assert_eq!(result, "Hello world!");
    println!("{}", result);
}
