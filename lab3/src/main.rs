fn swap(x: &mut i32, y: &mut i32){
    let tmp= *x;
    *x = *y;
    *y = tmp;

}

fn main() {
    let mut x:i32 = 4;
    let mut y:i32 = 5;
    println!("{} przed {}", x, y);
    swap(&mut x, &mut y);
    println!("{} po {}", x, y);

}
