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

fn generate_random_value_between(seed: &mut i32, start: i32, end: i32) -> i32 {
    *seed = (75 * *seed + 74) % (2_i32.pow(16) + 1);
    *seed % (end - start + 1) + start
}

fn swap_arr(arr: &mut[i32], i: usize, j:usize) {
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

fn rand_perm(arr: &mut[i32], seed: &mut i32){
    println!("{}", arr.len());
    for i in 0..arr.len(){
        let j = generate_random_value_between(seed, 0, arr.len() as i32);
        swap_arr(arr, i, j as usize)
    }
    *seed = (75 * *seed + 74) % (2_i32.pow(16) + 1);
}

fn main() {
    let mut arr: [i32; 5] = [1, 2, 3, 4 , 5];
    let mut ziarno = 100;
    rand_perm(&mut arr, &mut ziarno);


}
