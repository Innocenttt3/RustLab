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
            if let Some(final_result) = wartosc_syst2(&result_syst2) {
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



struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    fn from_3u8(red: u8, green: u8, blue: u8) -> RGB {
        RGB { r: red, g: green, b: blue }
    }

    fn from_3percent(red: f32, green: f32, blue: f32) -> Option<RGB> {
        if red < 0.0 || red > 100.0 || green < 0.0 || green > 100.0 || blue < 0.0 || blue > 100.0 {
            return None;
        }
        let r = ((red / 100.0) * 255.0) as u8;
        let g = ((green / 100.0) * 255.0) as u8;
        let b = ((blue / 100.0) * 255.0) as u8;
        Some(RGB { r, g, b })
    }

    fn black() -> RGB {
        RGB {r: 0, g: 0, b: 0}
    }
}




fn main() {
    let color = RGB::from_3u8(255, 100, 50);
    println!("RGB({}, {}, {})", color.r, color.g, color.b);
    let szary2 = RGB::from_3percent(50.0, 50.0, 50.0).unwrap();
    println!("RGB({}, {}, {})", szary2.r, szary2.g, szary2.b);
    let black = RGB::black();
    println!("RGB({}, {}, {})", black.r, black.g, black.b);
}
