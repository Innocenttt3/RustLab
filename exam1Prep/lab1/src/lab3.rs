fn swap(x: &mut i32, y: &mut i32) {
    let tmp = *x;
    *x = *y;
    *y = tmp;
}
fn swap_triple(x: &mut i32, y: &mut i32, z: &mut i32) {
    if x > y {
        swap (x, y)
    }
    if y > z {
        swap (y, z)
    }
}
fn swap_arr(arr: &mut [i32], i: usize, j:usize){
    let tmp = arr[i];
    arr[i] = arr[j];
    arr[j] = tmp;

}
fn main() {
    let mut arr: [i32; 3] = [1, 5, 10];
    println!("przed {} {} {}", arr[0], arr[1], arr[2]);
    swap_arr(&mut arr, 0, 2);
    println!("po {} {} {}", arr[0], arr[1], arr[2]);

}