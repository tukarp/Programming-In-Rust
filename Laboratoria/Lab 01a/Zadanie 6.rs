// Oblicz silnię
fn factorial(number: i64) -> i64 {
    if number <= 1 {
        return number;
    }
    return number * factorial(number - 1);
}

fn main() {
    let number = 5;
    let factorial_result : i64 = factorial(number);
    println!("Silnia z liczby {number}, wynosi: {factorial_result}!");
}
