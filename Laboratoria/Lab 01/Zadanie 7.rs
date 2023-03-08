fn main() {
    let mut x = 135790;
    
    while x > 0 {
        print!("{} ", x % 10);
        x /= 10;    
    }
}
