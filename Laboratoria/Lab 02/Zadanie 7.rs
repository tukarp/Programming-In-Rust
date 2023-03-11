// wyświetl cyfry podanej liczby
fn print_digits_of_number(mut number: i32) {
    while number > 0 {
        print!("{} ", number % 10);
        number /= 10;    
    }
}

fn main() {
    let number = 1234567890;
    print_digits_of_number(number);
}
