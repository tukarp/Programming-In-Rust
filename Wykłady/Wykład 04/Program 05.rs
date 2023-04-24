fn main() {
    let mut string1 = "Ala ma kota".to_string();
    
    string1.push_str(" i psa");
    string1.push('.');
    println!("{}", s1);
    
    println!("{:?}", string1.find('a'));
    println!("{:?}", string1.find('x'));
    println!("{:?}", string1.find("a"));
    println!("{:?}", string1.find("kot"));
    
    let string4 = string1.replace("kota", "szczura");
    println!("{}", s4);
    
    let a = string1.as_bytes();
    println!("{:?}", a);
}
