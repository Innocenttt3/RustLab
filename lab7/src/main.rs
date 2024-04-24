struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    fn from_3u8(r: u8, g: u8, b: u8) -> Rgb {
        Rgb { r, g, b }
    }

    fn from_3percent(r: f32, g: f32, b: f32) -> Option<Rgb> {
        if r < 0.0 || r > 100.0 || g < 0.0 || g > 100.0 || b < 0.0 || b > 100.0 {
            return None;
        }
        let r = ((r / 100.0) * 255.0) as u8;
        let g = ((g / 100.0) * 255.0) as u8;
        let b = ((b / 100.0) * 255.0) as u8;
        Some(Rgb { r, g, b })
    }

    fn gray(intensity: f32) -> Option<Rgb> {
        if intensity < 0.0 || intensity > 100.0 {
            return None;
        }
        let val = ((intensity / 100.0) * 255.0) as u8;
        Some(Rgb { r: val, g: val, b: val })
    }

    fn white() -> Rgb {
        Rgb::from_3u8(255, 255, 255)
    }

    fn black() -> Rgb {
        Rgb::from_3u8(0, 0, 0)
    }

    fn invert(&mut self) {
        self.r = 255 - self.r;
        self.g = 255 - self.g;
        self.b = 255 - self.b;
    }
}

fn filter_short_strings(strings: Vec<&str>) -> Vec<String> {
    strings
        .into_iter()
        .filter(|s| s.len() < 4)
        .map(|s| s.to_string())
        .collect()
}

fn filter_no_a(strings: Vec<&str>) -> Vec<String> {
    strings
        .into_iter()
        .filter(|s| !s.contains('a') && !s.contains('A'))
        .map(|s| s.to_string())
        .collect()
}

fn filter_contains_digits(strings: Vec<&str>) -> Vec<String> {
    strings
        .into_iter()
        .filter(|s| s.chars().any(|c| c.is_digit(10)))
        .map(|s| s.to_string())
        .collect()
}

fn reverse_strings(strings: Vec<&str>) -> Vec<String> {
    strings
        .into_iter()
        .map(|s| s.chars().rev().collect())
        .collect()
}

fn filter_double_letter(strings: Vec<&str>) -> Vec<String> {
    strings
        .into_iter()
        .filter(|s| {
            s.chars()
                .zip(s.chars().skip(1))
                .any(|(a, b)| a == b && a.is_alphabetic())
        })
        .map(|s| s.to_string())
        .collect()
}

fn main() {
    let strings = vec![
        "abc", "defgh", "ijk", "lmno", "pqrstu", "vwxyz", "1234", "567", "890", "xyz123",
    ];

    println!("Short strings: {:?}", filter_short_strings(strings.clone()));
    println!("No 'a' or 'A': {:?}", filter_no_a(strings.clone()));
    println!("Contains digits: {:?}", filter_contains_digits(strings.clone()));
    println!("Reversed strings: {:?}", reverse_strings(strings.clone()));
    println!("Double letter: {:?}", filter_double_letter(strings.clone()));

    let alphabet: Vec<char> = ('a'..='z').collect();
    println!("Alphabet: {:?}", alphabet);

    let squares: Vec<i32> = (1..=10).map(|x| x * x).collect();
    println!("Squares: {:?}", squares);

    let powers_of_two: Vec<i32> = (0..10).map(|x| 2_i32.pow(x)).collect();
    println!("Powers of two: {:?}", powers_of_two);

    let reciprocals: Vec<f64> = (1..=20).map(|x| 1.0 / (x as f64)).collect();
    println!("Reciprocals: {:?}", reciprocals);

    let divisible_by_3_not_by_4: Vec<i32> = (1..=100).filter(|x| x % 3 == 0 && x % 4 != 0).collect();
    println!("Divisible by 3 but not by 4: {:?}", divisible_by_3_not_by_4);
}




