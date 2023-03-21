fn main() {
    let a: i32 = 12;
    let b: u64 = 89;
    println!("{}", a as u64 + b);
    
    let a: f32 = 1.2;
    let b: f64 = 8.9;
    println!("{}", a + b as f32);
    
    println!("{}", 'z' as u8);
    println!("{}", 112 as char);
}
