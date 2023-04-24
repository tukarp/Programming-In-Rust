fn main() {
    let mut vector1 = Vec::new();
    println!("{:?} {:?} {:?}", vector1, vector1.len(), vector1.capacity());
    vector1.push('x');
    println!("{:?} {:?} {:?}", vector1, vector1.len(), vector1.capacity());
    let x = vector1.pop();
    println!("{:?} {:?} {:?} {:?}", vector1, vector1.len(), vector1.capacity(), x);
    
    let vector2 = Vec::<char>::with_capacity(100);
    println!("{:?} {:?} {:?}", vector2, vector2.len(), vector2.capacity());
    
    let vector3 = vec![3; 50];
    println!("{:?} {:?} {:?}", vector3, vector3.len(), vector3.capacity());
    
    let vector4 = vec![3, 23098, 43, 22, 12123, 5];
    println!("{:?} {:?} {:?}", vector4, vector4.len(), vector4.capacity());
    
    println!("{:?}", vector1 == vector2);
    println!("{:?}", vector3 == vector4);
    
    println!("{:?}", vector1 < vector2);
    println!("{:?}", vector3 < vector4);
    
    // dygresja
    println!("{}", (1, 'x') < (3, 'a'));
    println!("{}", ('x', 1) < ('a', 3));
}