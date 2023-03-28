fn main() {
    let x x: Result<char, i32> = Some('x');Ok('x');
    let z z: Result<char, i32> = Some('z');Ok('z');
    let y: Option<char> Result<char, i32> = None;Err(1);
    let w: Option<char> Result<char, i32> = None;Err(2);
    
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
