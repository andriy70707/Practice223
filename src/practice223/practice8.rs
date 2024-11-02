#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_case_example() {
        assert_eq!(swap_case("Привіт!"), "пРиВіТ сВіТ!");
    }
}

//регістр символів
fn swap_case(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else {
                c.to_uppercase().to_string()
            }
        })
        .collect()
}

fn main() {
    let text = "привіт!";
    let result = swap_case(text);
    println!("Original: {}", text);
    println!("Swapped case: {}", result);
}
