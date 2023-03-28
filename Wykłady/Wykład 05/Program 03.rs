fn main() {
    let x = Some('x');
    let z = Some('z');
    let y: Option<char> = None;
    
    println!("{:?}", x);
    println!("{:?}", y);
    
    println!("{:?}", x == y);
    println!("{:?}", x == z);
    
    println!("{:?}", y < x);
    println!("{:?}", x < z);
    
    println!("{:?}", x.is_none());
    println!("{:?}", x.is_some());
    println!("{:?}", y.is_none());
    println!("{:?}", y.is_some());
    
    println!("{:?}", x.unwrap());
//  println!("{:?}", y.unwrap());
    println!("{:?}", x.unwrap_or('?'));
    println!("{:?}", y.unwrap_or('?'));
    println!("{:?}", x.unwrap_or_default());
    println!("{:?}", y.unwrap_or_default());

    println!("{:?}", x.expect("spodziewam się znaku!"));
//  println!("{:?}", y.expect("spodziewam się znaku!"));
}
