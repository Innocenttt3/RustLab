fn zamien_syst8_na_syst2(z: &str) -> Option<String> {
    let mut result = String::new();
    for single_letter in z.chars() {
        match single_letter {
            '0' => continue,
            '1' => result.push_str("001"),
            '2' => result.push_str("010"),
            '3' => result.push_str("011"),
            '4' => result.push_str("100"),
            '5' => result.push_str("101"),
            '6' => result.push_str("110"),
            '7' => result.push_str("111"),
            _ => return None,
        }
    }
    if z.is_empty() { None } else { return Some(result); }
}

fn wartosc_syst2(z: &str) -> Option<u8> {
    let mut index = 0;
    let mut result = 0;
    for single_letter in z.chars().rev() {
        match single_letter {
            '0' => (),
            '1' => result += 2_u8.pow(index),
             _ => return None,
        }
        index += 1;
    }
    if z.is_empty() { None } else { Some(result)}
}


fn main() {
    println!("{:?}", wartosc_syst2("110"));
}
