
fn main() {
    //zad1
    // let  input = 6;
    // let mut result = 1;
    // let mut iterator = 1;
    // loop {
    //     result *= iterator;
    //     iterator += 1;
    //     if iterator == input + 1 {
    //         break;
    //     }
    // }
    // println!("{}", result)

    //zad2
    let input = 5;
    let mut result = 1;
    for i in 1..=input {
        result *= i;
    }
    println!("{}",result)
}