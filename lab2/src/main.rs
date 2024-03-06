mod test;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 5, y: 10 };

    // Użycie składni {:?}
    println!("{:?}", p);

    // Bez użycia składni {:?}
    println!("{}", p.x);
    println!("{}", p.y);
}
