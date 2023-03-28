fn main() {
    let s0 = "Witaj, świecie!";
    
    for c in s0.chars() {
        println!("{}", c);
    }
    
    for c in s0.bytes() {
        println!("{}", c);
    }
}
