fn main() {

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

}
