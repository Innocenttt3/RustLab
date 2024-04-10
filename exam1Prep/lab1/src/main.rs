fn swap_values(&x: i32, &y: i32){
    let mut tmp;
    tmp = *x;
    x = y;
    *y = tmp;
}
fn main() {
    let x = 10;
    let y = 5;
    println!("przed {} oraz  {}", x, y);
    swap_values(x, y);
    println!("po {} oraz  {}", x, y);

}