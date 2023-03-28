fn main() {
    let s0 = "Witaj, świecie!";
    let mut s2 = String::new();
    
    for c in s0.chars().rev() {
        s2.push(c);
    }
    println!("{}", s2);
}
