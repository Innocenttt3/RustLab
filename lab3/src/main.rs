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

fn generate_random_value_between(seed: &mut i32, start: i32, end: i32) -> i128 {
    *seed = (75 * *seed + 74) % (2_i32.pow(16) + 1);
    (*seed % (end - start + 1) + start) as i128
}


fn main() {
    let mut ziarno = 103;
    println!("{}", generate_random_value_between(&mut ziarno, 1, 6));
    println!("{}", generate_random_value_between(&mut ziarno, 1, 6));
    println!("{}", generate_random_value_between(&mut ziarno, 1, 6));
    println!("{}", generate_random_value_between(&mut ziarno, 1, 6));
    println!("{}", generate_random_value_between(&mut ziarno, 1, 6));
    println!("{}", generate_random_value_between(&mut ziarno, 1, 6));
}
