mod test;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 5, y: 10 };

    // Użycie składni {:?}
    println!("Wartość punktu testowa: {:?}", p);

    // Bez użycia składni {:?}
    println!("Wartość x: {}", p.x);
    println!("Wartość y: {}", p.y);
}
