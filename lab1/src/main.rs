fn main() {

    let mut input_value = 12345;
    let mut value;
    while input_value > 0 {
        value = input_value % 10;
        print!("{}", value);
        input_value /= 10;
    }
}
