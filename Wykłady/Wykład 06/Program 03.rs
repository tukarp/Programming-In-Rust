fn main() {
    let mut v1 = Vec::new();
    println!("{:?} {:?} {:?}", v1, v1.len(), v1.capacity());
    v1.push('x');
    println!("{:?} {:?} {:?}", v1, v1.len(), v1.capacity());
    let x = v1.pop();
    println!("{:?} {:?} {:?} {:?}", v1, v1.len(), v1.capacity(), x);
    
    let v2 = Vec::<char>::with_capacity(100);
    println!("{:?} {:?} {:?}", v2, v2.len(), v2.capacity());
    
    let v3 = vec![3; 50];
    println!("{:?} {:?} {:?}", v3, v3.len(), v3.capacity());
    
    let v4 = vec![3, 23098, 43, 22, 12123, 5];
    println!("{:?} {:?} {:?}", v4, v4.len(), v4.capacity());
    
    println!("{:?}", v1 == v2);
    println!("{:?}", v3 == v4);
    
    println!("{:?}", v1 < v2);
    println!("{:?}", v3 < v4);
    
    // dygresja
    println!("{}", (1, 'x') < (3, 'a'));
    println!("{}", ('x', 1) < ('a', 3));
}