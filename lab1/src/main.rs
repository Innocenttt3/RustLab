fn main() {
    let mut desired_value = 5;
    let mut result = 1;
    while desired_value > 0 {
        result *= desired_value;
        desired_value -= 1;
    }
    println!("{}", result);
}
