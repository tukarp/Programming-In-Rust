fn main() {
    let x: Result<char, i32> = Ok('x');
    let z: Result<char, i32> = Ok('z');
    let y: Result<char, i32> = Err(1);
    
    println!("{:?}", x);
    println!("{:?}", y);
    
    println!("{:?}", x == y);
    println!("{:?}", x == z);
    
    println!("{:?}", y < x);
    println!("{:?}", x < z);
    
    println!("{:?}", x.is_err());
    println!("{:?}", x.is_ok());
    println!("{:?}", y.is_err());
    println!("{:?}", y.is_ok());
    
    println!("{:?}", x.unwrap());
//  println!("{:?}", y.unwrap());
    println!("{:?}", x.unwrap_or('?'));
    println!("{:?}", y.unwrap_or('?'));
    println!("{:?}", x.unwrap_or_default());
    println!("{:?}", y.unwrap_or_default());

    println!("{:?}", x.expect("spodziewam się znaku!"));
//  println!("{:?}", y.expect("spodziewam się znaku!"));

    println!("{:?}", x.ok());
    println!("{:?}", z.ok());
    println!("{:?}", y.ok());
    println!("{:?}", x.err());
    println!("{:?}", z.err());
    println!("{:?}", y.err());
}
