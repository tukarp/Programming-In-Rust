fn factorial(number: i64) {
    let mut result = 1;
    let mut i = 1;
    while i <= number {
        result *= i;
        i += 1;
    }
    println!("Silnia z liczby {number}, wynosi: {result}!");
}

fn main() {
    let number = 5;
    
    factorial(number);
}
