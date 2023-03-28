fn main() {
    let mut s1 = "Ala ma kota".to_string();
    
    s1.push_str(" i psa");
    s1.push('.');
    println!("{}", s1);
    
    println!("{:?}", s1.find('a'));
    println!("{:?}", s1.find('x'));
    println!("{:?}", s1.find("a"));
    println!("{:?}", s1.find("kot"));
    
    let s4 = s1.replace("kota", "szczura");
    println!("{}", s4);
    
    let a = s1.as_bytes();
    println!("{:?}", a);
}
