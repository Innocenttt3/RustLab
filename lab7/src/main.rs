fn main() {
    let alphabet: Vec<char> = ('a'..='z').collect();
    println!("Alphabet: {:?}", alphabet);

    let squares: Vec<i32> = (1..=10).map(|x| x * x).collect();
    println!("Squares: {:?}", squares);

    let powers_of_two: Vec<i32> = (0..10).map(|x| 2_i32.pow(x)).collect();
    println!("Powers of two: {:?}", powers_of_two);

    let reciprocals: Vec<f64> = (1..=20).map(|x| 1.0 / (x as f64)).collect();
    println!("Reciprocals: {:?}", reciprocals);

    let divisible_by_3_not_by_4: Vec<i32> = (1..=100).filter(|x| x % 3 == 0 && x % 4 != 0).collect();
    println!("Divisible by 3 but not by 4: {:?}", divisible_by_3_not_by_4);




}
