fn zamien_syst8_na_syst2(z: &str) -> Option<String>{
    if z.is_empty() {return None}
    let mut result = String::new();
    for single_letter in z.chars(){
        match single_letter {
            '0' => result.push_str("000"),
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
    let trimmed_result = result.trim_start_matches('0');
    return Some(trimmed_result.to_string())

}

fn wartosc_syst2(z: &str) -> Option<u8> {
    let mut result = 0;
    let mut index = 0;
    if z.len() > 8 {return None}
    else {
        for single_number in z.chars().rev() {
            match single_number {
                '0' => (),
                '1' => result += 2_u8.pow(index),
                _ => return None,
            }
            index += 1;
        }
        return Some(result)}
}

fn wartosc_syst8(z: &str) -> Option<u8> {
    if let Some(result_syst2) = zamien_syst8_na_syst2(z) {
            if (let Some(final_result) = wartosc_syst2(&result_syst2)) {
                return Some(final_result)
            } else {
                return None
            }
    } else {
        return None
    }
}

fn wartosc_cyfry(c: char) -> Result<u8, String> {
    match c {
        '0' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        _ => Err(format!("'{}' is not a digit.", c)),
    }
}

fn dodaj_pisemnie(a; &str, b: &str) -> Result<String, String>{
    let mut a = 0;
    let mut b = 0;
    let mut tmp = 0;
    for single_number in a.chars(){
        if let Some(tmp) = wartosc_cyfry(single_number){
            a += tmp;
        } else {return Err(format!("'{}' is not a digit.", c))}

    }
}

fn main(){
    if let Some(result) = wartosc_syst8("111"){
        println!("Number value: {}", result);
    } else {
        println!("Wrong input");
    }

}
