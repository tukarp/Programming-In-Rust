// Zamień celsjusze na fahrenheity
fn celsius_to_fahrenheit(celsius_temperature: f32) -> f32 {
    let fahrenheit_temperature: f32 = ((9.0 / 5.0) * celsius_temperature) + 32.0;
    return fahrenheit_temperature;
}

fn main() {
    let celsius_temperature: f32 = 15.0;
    let fahrenheit_temperature: f32 = celsius_to_fahrenheit(celsius_temperature);
    println!("Temperatura {celsius_temperature} stopni Celsjusza to {fahrenheit_temperature} stopni Fahrenheita!");
}
