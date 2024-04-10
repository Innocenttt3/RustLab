fn test(x: i32) -> Option<i32> {
    if x < 0 {
        Some(0)
    } else if x >= 0 && x < 4 {
        Some(x)
    } else if x >= 4 {
        Some(2 * x)
    } else {
        None
    }
}

fn main() {
    if let Some(result) = test(3) {
        println!("{}", result);
    } else {
        println!("Brak wyniku dla podanej wartości x");
    }
}