fn main() {
    let x = Some('x');
    let z = Some('z');
    let y: Option<char> = None;
    let w: Option<char> = None;
    
    println!("{:?}", x.and(z));
    println!("{:?}", x.and(y));
    println!("{:?}", w.and(z));
    println!("{:?}", w.and(y));

    println!("{:?}", x.or(z));
    println!("{:?}", x.or(y));
    println!("{:?}", w.or(z));
    println!("{:?}", w.or(y));

    println!("{:?}", x.xor(z));
    println!("{:?}", x.xor(y));
    println!("{:?}", w.xor(z));
    println!("{:?}", w.xor(y));
}
