fn main() {
    //zad1
        let year_input = 2023;
        let checker_value = 4;
        if (year_input % 4 == 0 && year_input % 100 == 0) || year_input % 400 != 0 {
            println!("jest przestepny")
        } else {
            println!("nie jest przestepny")
        }
    //zad2
    let year_input = 2023;
    let checker_value = 4;
    let month_input = 2;
    if (year_input % 4 == 0 && year_input % 100 == 0) || year_input % 400 != 0 {
        println!("jest przestepny");
        match month_input {
            1 => println!("31"),
            2 => println!("29"),
            3 => println!("31"),
            4 => println!("30"),
            5 => println!("31"),
            6 => println!("30"),
            7 => println!("31"),
            8 => println!("31"),
            9 => println!("30"),
            10 => println!("31"),
            11 => println!("30"),
            12 => println!("31"),
            _ => println!("Month out of bands"),
        }
    } else {
        println!("nie jest przestepny");
        match month_input {
            1 => println!("31"),
            2 => println!("29"),
            3 => println!("31"),
            4 => println!("30"),
            5 => println!("31"),
            6 => println!("30"),
            7 => println!("31"),
            8 => println!("31"),
            9 => println!("30"),
            10 => println!("31"),
            11 => println!("30"),
            12 => println!("31"),
            _ => println!("Month out of bands"),
        }
    }
    //zad3
    let temp_celcius_input:f32 = 10.0;
    let temp_farenheit:f32 = 32.0 + (9.0 / 5.0) * temp_celcius_input;
    println!("{}", temp_farenheit);
    //zad4
    let temp_farenheit_input:f32 = 50.0;
    let temp_celcius:f32 = (temp_farenheit_input - 32.0) * 5.0 / 9.0;
    println!("{}", temp_celcius);
    //zad5
    let hours_start = 15;
    let minutes_start = 30;
    let seconds_start = 45;

    let hours_end = 16;
    let minutes_end = 25;
    let seconds_end = 47;

    let result_hour = hours_end - hours_start;
    let  mut final_result_hour;
    if result_hour < 0 {
        final_result_hour = 24 + result_hour;
    } else {
        final_result_hour = result_hour;
    }
    let result_minute = minutes_end - minutes_start;
    let mut final_result_minute;
    if result_minute < 0 {
        final_result_hour -= 1;
        final_result_minute = 60 + result_minute;
    } else {
        final_result_minute = result_minute;
    }
    let result_seconds = seconds_end - seconds_start;
    let mut final_result_seconds;
    if result_seconds < 0 {
        final_result_minute -= 1;
        final_result_seconds = 60 + result_seconds;
    } else {
        final_result_seconds = result_seconds;
    }
    println!("{}:{}:{}", final_result_hour, final_result_minute, final_result_seconds);
    //zad6
    let mut desired_value = 5;
    let mut result = 1;
    while desired_value > 0 {
        result *= desired_value;
        desired_value -= 1;
    }
    println!("{}", result);
    //zad7
    let mut input_value = 12345;
    let mut value;
    while input_value > 0 {
        value = input_value % 10;
        print!("{}", value);
        input_value /= 10;
    }
    //zad8
    let mut input_value = 12345;
    let mut value = 0;
    while input_value > 0 {
        value = input_value % 10;
        print!("{}", value);
        value += input_value % 10;
        input_value /= 10;
    }
    println!("{}", value);


    //zad9
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
