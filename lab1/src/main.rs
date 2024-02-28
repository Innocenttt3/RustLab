fn main() {
    zad1
    let year_input = 2023;
    let checker_value = 4;
    if (year_input % 4 == 0 && year_input % 100 == 0) || year_input % 400 != 0 {
        println!("jest przestepny")
    } else {
        println!("nie jest przestepny")
    }
}
