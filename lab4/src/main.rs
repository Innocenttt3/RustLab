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
            result -= previous;
            result += current - previous;
        } else {
            result += current;
        }
        previous = current;
    }
    result
}

fn co_drugi_znak(napis: &str) -> String {
    let mut result: String = "".to_string();
    let mut iterator = 0;
    for single_letter in napis.chars() {
        if iterator % 2 == 0 {
            result.push(single_letter);
        }
        iterator += 1;
    }
    return result;
    //bledne podejscie
}

fn co_drugi_znak_better(napis: &str) -> String {
    napis.chars().step_by(2).collect()
}

fn wizytowka(imie: &str, nazwisko: &str) -> String {
    let mut result = String::new();
    if let Some(pierwsza_litera) = imie.chars().next() {
        let pierwsza_litera_duza = pierwsza_litera.to_uppercase();
        result.push_str(&pierwsza_litera_duza.to_string());
    }
    result.push_str(". ");
    result.push_str(nazwisko);
    result
}

fn main() {
    println!("{}", wizytowka("kamil", "Golawski"));
}
