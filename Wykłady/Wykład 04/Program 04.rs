fn main() {
    let s0 = "Witaj, świecie!";
    let mut s1 = "Ala ma kota".to_string();
    let s2 = String::new();
    let s3 = String::from("Pies i kot.");
    
    s1.push_str(" i psa");
    s1.push('.');
    println!("{}", s1);
    
    println!("{:?}", s0.get(..3));
    println!("{:?}", s0.get(3..));
    println!("{:?}", s0.get(1..8));
    println!("{:?}", s0.get(180..));

    println!("{:?}", s0.chars().nth(6));
    println!("{:?}", s0.bytes().nth(6));
    println!("{:?}", s0.chars().nth(7));
    println!("{:?}", s0.bytes().nth(7));
    println!("{:?}", s0.chars().nth(8));
    println!("{:?}", s0.bytes().nth(8));
    println!("{:?}", s0.chars().nth(9));
    println!("{:?}", s0.bytes().nth(9));
    println!("{:?}", s0.chars().nth(99));
    println!("{:?}", s0.bytes().nth(99));

    println!("{:?}", s0.len());
    println!("{:?}", s0.bytes().len());
//  println!("{:?}", s0.chars().len());     // nie ma
}
