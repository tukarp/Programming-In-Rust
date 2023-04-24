fn main() {
    let string1 = "Witaj, świecie!";
    let mut string2 = String::new();
    
    for char in string1.chars().rev() {
        string2.push(char);
    }
    println!("{}", string2);
}
