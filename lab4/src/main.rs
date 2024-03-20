fn liczba_wystapien(napis: &str, znak: char) -> usize {
    let mut counter = 0;
    let uniwersalny = napis.to_lowercase();
    for tmp in uniwersalny.chars() {
        if tmp == znak {
            counter += 1;
        }
    }
    return counter;
}


fn rzymskie(napis: &str) -> u128 {
    let mut result = 0;
    let mut previous = 0;
    let mut current = 0;
    for single_letter in napis.chars() {
        match single_letter {
            'I' => current = 1,
            'V' => current = 5,
            'X' => current = 10,
            'L' => current = 50,
            'C' => current = 100,
            'D' => current = 500,
            'M' => current = 1000,
            _ => println!("Can't recognize char")
        }
        if previous < current {
            println!("{}, {}", previous, current - previous);
            result -= previous;
            result += current - previous;
        } else {
            result += current;
        }
        previous = current;
    }
    result
}



fn main() {
    println!("{}", rzymskie("CCVII"))
}
