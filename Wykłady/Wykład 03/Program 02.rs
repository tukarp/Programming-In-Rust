fn main() {
    println!("{}", i32::MAX);
    println!("{}", i64::MIN);
    println!("{}", u8::MAX);
    println!("{}", u32::MIN);

    println!("{}", f32::MAX);
    println!("{}", f32::MIN);
    println!("{}", f64::MAX);
    println!("{}", f64::MIN);
    println!("{}", f32::MIN_POSITIVE);
    println!("{}", f32::NAN);
    println!("{}", f32::INFINITY);
    
    println!("{}", usize::MIN);
    println!("{}", usize::MAX);
    println!("{}", isize::MIN);
    println!("{}", isize::MAX);

    println!("{}", usize::MAX as u128 == u64::MAX as u128);
    println!("{}", usize::MAX as u128 == u32::MAX as u128);
}
