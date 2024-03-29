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

fn swap_arr(arr: &mut [i32], i: usize, j: usize) {
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

fn rand_perm(arr: &mut [i32], seed: &mut i32) {
    for i in 0..arr.len() {
        *seed = (75 * *seed + 74) % (2_i32.pow(16) + 1);
        let j = generate_random_value_between(seed, 0, (arr.len() - 1) as i32);
        swap_arr(arr, i, j as usize)
    }
}

// fn count_occurance(napis: &str, sign: char) -> i32 {
//     let mut result = 0;
//     for i in 0..napis.len() {
//         if napis == sign {
//             result += 1;
//         }
//     }
//     result
// }

fn main() {
    let napis = "testowe";
    let sign = "t";
    //println!("{}", count_occurance(napis, sign.parse().unwrap()))
}
