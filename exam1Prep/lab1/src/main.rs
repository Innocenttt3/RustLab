fn co_drugi_znak(napis: &str) -> String {
    let result = napis.chars().step_by(2).collect();
    result
}

fn szyfruj(napis: &str, value: usize) -> String {
    let result: String = napis.chars().take(value).collect();
    let mut finalresult: String = result.chars().rev().collect();
    let rest: String = napis.chars().skip(2).collect();
    finalresult += &rest;
    finalresult
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

fn dodaj_pisemnie(a: &str, b: &str) -> String {
    let liczba_a: f64 = a.parse().unwrap();
    let liczba_b: f64 = b.parse().unwrap();
    println!("{}", liczba_a);
    println!("{}", liczba_b);
    let suma = liczba_a + liczba_b;

    suma.to_string()
}

fn match_test(a: u32) -> String {
    let mut result = String::new();
    match a {
        5 => result = "gang".to_string(),
        1 => result = "test".to_string(),
        _ => println!("nie znana liczba!")
    }
    result
}

fn main() {
    println!("{}", match_test(5));
}
