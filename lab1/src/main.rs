fn main() {

    let mut input_value = 12345;
    let mut value = 0;
    while input_value > 0 {
        value += input_value % 10;
        input_value /= 10;
    }
    println!("{}", value);
}
