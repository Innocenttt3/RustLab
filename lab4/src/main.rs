fn liczba_wystapien(napis: &str, znak: char) -> i32{
    let mut counter = 0;
    for tmp in napis.chars(){
        if tmp == znak {
            counter += 1;
        }
    }
    return counter;
}

fn main() {
    println!("{}", liczba_wystapien("testowy tekst", 't'))
}
