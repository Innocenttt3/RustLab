
fn main() {
    let  input = 6;
    let mut result = 1;
    let mut iterator = 1;
    loop {
        result *= iterator;
        iterator += 1;
        if iterator == input + 1 {
            break;
        }
    }
    println!("{}", result)
}