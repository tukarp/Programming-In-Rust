// oblicz sumę cyfr podanej liczby
fn get_digits_sum(mut number: i32) -> i32{
    let mut sum: i32 = 0;
    while number > 0 {
        sum = sum + number % 10;
        number /= 10;
    }
    return sum;
}

fn main() {
    let number = 1234567890;
    let digits_sum = get_digits_sum(number);
    println!("Suma cyfr liczby {number} wynosi: {digits_sum}");
}
