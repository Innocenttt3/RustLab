fn main() {
    let temp_farenheit_input:f32 = 50.0;
    let temp_celcius:f32 = (temp_farenheit_input - 32.0) * 5.0 / 9.0;
    println!("{}", temp_celcius);
}
