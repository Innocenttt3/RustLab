enum Card_type {
    Pik,
    Kier,
    Karo,
    Trefl
}

enum File_error {
    No_error,
    Wrong_file_extension,
    File_not_found(String),
    Too_large_file(u64, u64)
}

impl File_error {
    fn show_error(&self) {
        match self {
            File_error::No_error => println!("No issues were found"),
            File_error::Wrong_file_extension => println!("Wrong file extension"),
            File_error::File_not_found(file_name) => println!("File: {} was not found in that directory", file_name),
            File_error::Too_large_file(actual_size, min_size) => println!("File size: {} is over limit: {}", actual_size, min_size)
        }
    }
}

#[derive(Debug)]
struct RandGen {
    seed: i32
}

impl RandGen {
    fn new(new_seed: i32) -> RandGen { RandGen { seed: new_seed } }

    fn gen_range(&mut self, start: i32, end: i32) -> i32 {
        self.seed = (75 * self.seed + 74) % (2_i32.pow(16) + 1);
        self.seed % (end - start + 1) + start
    }

    fn pokaz_komunikat(&self, value: i32, start: i32, end: i32) {
        println!("Wylosowana liczba: {}", value);
        println!("Czy wartość jest większa lub równa {}? {}", start, value >= start);
        println!("Czy wartość jest mniejsza lub równa {}? {}", end, value <= end);
    }
}

#[derive(Debug)]
struct Urna {
    letters: Vec<char>,
    generator: RandGen
}

impl Urna {
    fn new(generator: RandGen) -> Urna {
        Urna { letters: Vec::new(), generator }
    }

    fn doloz(&mut self, letter: char) {
        self.letters.push(letter);
    }

    fn losuj_z_us(&mut self) -> Option<char> {
        if self.letters.is_empty() {
            None
        } else {
            let idx = self.generator.gen_range(0, self.letters.len() as i32 - 1) as usize;
            Some(self.letters[idx])
        }
    }

    fn losuj_bez_us(&mut self) -> Option<char> {
        if self.letters.is_empty() {
            None
        } else {
            let idx = self.generator.gen_range(0, self.letters.len() as i32 - 1) as usize;
            Some(self.letters.remove(idx))
        }
    }
}

fn main() {
    let mut urna = Urna::new(RandGen::new(123));

    let a: Option<char> = urna.losuj_z_us();
    println!("{}", a.is_none());
    let a: Option<char> = urna.losuj_bez_us();
    println!("{}", a.is_none());

    urna.doloz('a');
    urna.doloz('b');
    urna.doloz('c');
    urna.doloz('d');

    let b: Option<char> = urna.losuj_z_us();
    println!("{:?}", b);

    let c: Option<char> = urna.losuj_bez_us();
    println!("{:?}", c);

    println!("{:?}", urna); 
}
