fn is_leap_year(year: u32) -> bool{
    if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0){
        return true;
    }
    return false;

}

fn return_day_amount(month: u32, year: u32) -> u32{
    let months: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if is_leap_year(year) == true && month == 2 {
       29
    } else {
        months[(month - 1) as usize]
    }
}

fn celsius_to_fahrenheit(temp: u32) -> f32{
    32.0 + (9.0 / 5.0) * (temp as f32)
}

fn factorial(mut value: u32) -> u32{
    let mut result = 1;
    while value > 0 {
        result *= value;
        value -= 1;
    }
    return result;

}


fn factorial_recursion(mut value: u32, mut result: u32) -> u32{
    if value == 0 {
        result
    } else {
        result *= value;
        factorial_recursion(value - 1, result)
    }

}

fn main() {
    println!("{}", factorial_recursion(5, 1));


}