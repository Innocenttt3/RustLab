fn main() {
    let a: i32 = 10;
    let b: i32 = 11;
    let c: i32 = 12;
    for x in 1..a {
        for y in 1..b {
            for z in 1..c {
                if x.pow(2) + y.pow(2) == z.pow(2) {
                    println!("{} / {} / {}", x, y, z);
                }
            }
        }
    }
}
