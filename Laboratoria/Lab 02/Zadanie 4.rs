// zamień fahrenheity na celsjusze
fn fahrenheit_to_celsius(fahrenheit_temperature: f32) -> f32 {
    let celsius_temperature: f32 = (fahrenheit_temperature - 32.0) * (5.0 / 9.0);
    return celsius_temperature;
}

fn main() {
    let fahrenheit_temperature: f32 = 60.0;
    let celsius_temperature: f32 = fahrenheit_to_celsius(fahrenheit_temperature);
    println!("Temperatura {fahrenheit_temperature} stopni Fahrenheita to {celsius_temperature} stopni Celsjusza!");
}
