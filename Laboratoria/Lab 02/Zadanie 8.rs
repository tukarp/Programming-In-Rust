fn main() {
    let original_x = 135790;
    let mut x = original_x;
    let mut sum = 0;
    
    while x > 0 {
        sum = sum + x % 10;
        x /= 10;
    }
    println!("Suma cyfr liczby {original_x} wynosi: {sum}");
}
