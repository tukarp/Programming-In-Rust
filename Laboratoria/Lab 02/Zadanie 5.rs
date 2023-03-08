fn main() {
    // startowa godzina
    let mut hours_start: i32 = 10;
    let mut minutes_start: i32 = 45;
    let mut seconds_start: i32 = 39;
    // godzina od której odejmujemy
    let hours_subtract: i32 = 16;
    let minutes_subtract: i32 = 15;
    let seconds_subtract: i32 = 47;
    
    println!("Startowa godzina: {hours_start}:{minutes_start}:{seconds_start}, odejmowana godzina: {hours_subtract}:{minutes_subtract}:{seconds_subtract}");
    
    hours_start -= hours_subtract;
    minutes_start -= minutes_subtract;
    seconds_start -= seconds_subtract;
    if hours_start < 0 {
            hours_start *= -1;
    } if minutes_start < 0 {
            minutes_start *= -1;
    } if seconds_start < 0 {
            seconds_start *= -1;
    }
    println!("Różnica czasu: {hours_start} godziny, {minutes_start} minut, {seconds_start} sekund");
}
