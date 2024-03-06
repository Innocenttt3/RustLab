fn f(x: f64) -> f64 {
    return x.powf(2.0) - 1.0;
}
fn fp(x: f64) -> f64{
 return 2.0 * x;
}
fn met_newt_loop(x0: f64, eps: f64, n: u128) -> f64 {
    let mut result:f64 = x0;
    let mut i: u128 = 1;
 loop {
     result = result - (f(result) / fp(result));
     if i + 1 == n{
         break;
     }
     if f(result).abs() < eps {
         return result;
     }
     i += 1;
 }
    result
}
fn met_newt_while(x0: f64, eps: f64, n: u128) -> f64 {
    let mut result:f64 = x0;
    let mut i: u128 = 1;
    while i + 1 != n {
        result = result - (f(result) / fp(result));
        if f(result).abs() < eps {
            return result;
        }
        i += 1;
    }
    result
}
fn met_newt_recur(x0: f64, eps: f64, n: u128, mut result: f64) -> f64 {
    if result == 0.0 {
        result = x0;
    }
    if 1 == n {
        return result;
    }
    if f(result).abs() < eps {
        return result;
    }
    result = result - (f(result) / fp(result));
    return met_newt_recur(x0, eps, n - 1, result)
}
fn met_newt_for(x0: f64, eps: f64, n: u128) -> f64 {
    let mut result:f64 = x0;
    for i in 1..=n {
        result = result - (f(result) / fp(result));
        if i + 1 == n{
            break;
        }
        if f(result).abs() < eps {
            return result;
        }
    }
    result
}

fn main() {
    //zad1
    // let  input = 6;
    // let mut result = 1;
    // let mut iterator = 1;
    // loop {
    //     result *= iterator;
    //     iterator += 1;
    //     if iterator == input + 1 {
    //         break;
    //     }
    // }
    // println!("{}", result)

    //zad1.2
    // let input = 5;
    // let mut result = 1;
    // for i in 1..=input {
    //     result *= i;
    // }
    // println!("{}",result)

    //zad2
    let result = met_newt_for(10.0 , 0.000001, 10000);
    println!("{}", result)
}