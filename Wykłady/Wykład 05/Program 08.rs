fn main() {
    let x: Result<char, i32> = Ok('x');
    let z: Result<char, i32> = Ok('z');
    let y: Result<char, i32> = Err(1);
    let w: Result<char, i32> = Err(2);
    
    println!("{:?}", x.and(z));
    println!("{:?}", x.and(y));
    println!("{:?}", w.and(z));
    println!("{:?}", w.and(y));

    println!("{:?}", x.or(z));
    println!("{:?}", x.or(y));
    println!("{:?}", w.or(z));
    println!("{:?}", w.or(y));
}
