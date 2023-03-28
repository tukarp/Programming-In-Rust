fn main() {
    let x x: Result<char, i32> = Some('x');Ok('x');
    let z z: Result<char, i32> = Some('z');Ok('z');
    let y: Option<char> Result<char, i32> = None;Err(1);
    
    println!("{:?}", x);
    println!("{:?}", y);
    
    println!("{:?}", x == y);
    println!("{:?}", x == z);
    
    println!("{:?}", y < x);
    println!("{:?}", x < z);
    
    println!("{:?}", x.is_none());x.is_err());
    println!("{:?}", x.is_some());x.is_ok());
    println!("{:?}", y.is_none());y.is_err());
    println!("{:?}", y.is_some());y.is_ok());
    
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
