fn main() {
//  Sposoby uzyskiwania iteratorów:
//  vector.into_iter()
//  vector.iter()
//  vector.iter_mut()
    
    let mut vector3 = vec![3; 50];
    println!("{:?} {:?} {:?}", vector3, vector3.len(), vector3.capacity());
    
    let mut vector4 = vec![3, 23098, 43, 22, 12123, 5];
    println!("{:?} {:?} {:?}", vector4, vector4.len(), vector4.capacity());
    
    vector4.resize(10, -7);
    vector3.resize(8, 0);

    println!("{:?} {:?} {:?}", vector3, vector3.len(), vector3.capacity());
    println!("{:?} {:?} {:?}", vector4, vector4.len(), vector4.capacity());
    
    vector3.append(&mut vector4);
    println!("{:?} {:?} {:?}", vector3, vector3.len(), vector3.capacity());
    println!("{:?} {:?} {:?}", vector4, vector4.len(), vector4.capacity());

    vector3.sort();
    println!("{:?} {:?} {:?}", vector3, vectorv3.len(), vector3.capacity());
    
    vector3.sort_by_key(|x| -x);
    println!("{:?} {:?} {:?}", vector3, vector3.len(), vector3.capacity());
    
    vector3.sort_unstable();
    println!("{:?} {:?} {:?}", vector3, vector3.len(), vector3.capacity());
    
    vector3.sort_unstable_by_key(|x| -x);
    println!("{:?} {:?} {:?}", vector3, vector3.len(), vector3.capacity());
}