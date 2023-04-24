fn main() {
    let string = "Witaj, świecie!";
    let mut string1 = "Ala ma kota".to_string();
    let string2 = String::new();
    let string3 = String::from("Pies i kot.");
    
    string1.push_str(" i psa");
    string1.push('.');
    println!("{}", string1);
    
    println!("{:?}", string0.get(..3));
    println!("{:?}", string0.get(3..));
    println!("{:?}", string0.get(1..8));
    println!("{:?}", string0.get(180..));

    println!("{:?}", string0.chars().nth(6));
    println!("{:?}", string0.bytes().nth(6));
    println!("{:?}", string0.chars().nth(7));
    println!("{:?}", string0.bytes().nth(7));
    println!("{:?}", string0.chars().nth(8));
    println!("{:?}", string0.bytes().nth(8));
    println!("{:?}", string0.chars().nth(9));
    println!("{:?}", string0.bytes().nth(9));
    println!("{:?}", string0.chars().nth(99));
    println!("{:?}", string0.bytes().nth(99));

    println!("{:?}", string0.len());
    println!("{:?}", string0.bytes().len());
}
