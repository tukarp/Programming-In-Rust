fn main() {
    let string0 = "Witaj, świecie!";
    
    let string4: String = string0.chars().rev().collect();
    println!("{}", string4);
}
