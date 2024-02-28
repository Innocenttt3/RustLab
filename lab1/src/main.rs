fn main() {
    let year_input = 2023;
    let month_input = 2;
    if (year_input % 4 == 0 && year_input % 100 == 0) || year_input % 400 != 0 {
        match month_input {
            1 => println!("31"),
            2 => println!("29"),
            3 => println!("31"),
            4 => println!("30"),
            5 => println!("31"),
            6 => println!("30"),
            7 => println!("31"),
            8 => println!("31"),
            9 => println!("30"),
            10 => println!("31"),
            11 => println!("30"),
            12 => println!("31"),
            _ => println!("Month out of bands"),
        }
    } else {
        match month_input {
            1 => println!("31"),
            2 => println!("29"),
            3 => println!("31"),
            4 => println!("30"),
            5 => println!("31"),
            6 => println!("30"),
            7 => println!("31"),
            8 => println!("31"),
            9 => println!("30"),
            10 => println!("31"),
            11 => println!("30"),
            12 => println!("31"),
            _ => println!("Month out of bands"),
        }
    }
}
