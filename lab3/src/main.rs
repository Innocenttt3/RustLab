fn swap(x: &mut i32, y: &mut i32) {
    let tmp = *x;
    *x = *y;
    *y = tmp;
}

fn change_order(x: &mut i32, y: &mut i32, z: &mut i32) {
    if *x <= *y {
        swap(x, y);
    }
    if *y <= *z {
        swap(y, z);
    }
    if *x <= *y {
        swap(x, y);
    }
}

fn main() {
    let mut x: i32 = 2;
    let mut y: i32 = 6;
    let mut z: i32 = 3;
    println!("przed {} {} {}", x, y, z);
    change_order(&mut x, &mut y, &mut z);
    println!("po {} {} {}", x, y, z);
}
