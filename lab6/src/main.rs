fn zamien_syst8_na_syst2(z: &str) -> Option<String> {
    if z.is_empty() {
        return None;
    }

    let mut result = String::new();
    for single_letter in z.chars() {
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
    let trimmed_result = result.trim_start_matches('0').to_string();
    Some(trimmed_result)
}

fn wartosc_syst2(z: &str) -> Option<u8> {
    let mut result = 0;
    let mut index = 0;

    for single_letter in z.chars().rev() {
        match single_letter {
            '0' => (),
            '1' => result += 2_u8.pow(index),
            _ => return None,
        }
        index += 1;
    }

    Some(result)
}

fn wartosc_syst8(z: &str) -> Option<u8> {
    let napis_po_zmianie_option = zamien_syst8_na_syst2(z);

    match napis_po_zmianie_option {
        Some(napis_po_zmianie) => {
            if napis_po_zmianie.len() > 8 {
                return None;
            }
            let napis_po_zmianie_str: &str = &napis_po_zmianie;
            wartosc_syst2(napis_po_zmianie_str)
        }
        None => None,
    }
}

fn main() {
    println!("{:?}", wartosc_syst8("123"));
}
