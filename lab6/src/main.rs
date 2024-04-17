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
    if z.is_empty() {None} else {return Some(result)}
}


fn main() {
   println!("{:?}", zamien_syst8_na_syst2("63")) ;
}
