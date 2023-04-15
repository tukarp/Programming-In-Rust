fn main() {
    let mut v3 = vec![3; 50];
    println!("{:?} {:?} {:?}", v3, v3.len(), v3.capacity());
    
    let mut v4 = vec![3, 23098, 43, 22, 12123, 5];
    println!("{:?} {:?} {:?}", v4, v4.len(), v4.capacity());
    
    v4.resize(10, -7);
    v3.resize(8, 0);

    println!("{:?} {:?} {:?}", v3, v3.len(), v3.capacity());
    println!("{:?} {:?} {:?}", v4, v4.len(), v4.capacity());
    
    v3.append(&mut v4);
    println!("{:?} {:?} {:?}", v3, v3.len(), v3.capacity());
    println!("{:?} {:?} {:?}", v4, v4.len(), v4.capacity());

//  sposoby uzyskiwania iteratorów:
//  v3.into_iter()
//  v3.iter()
//  v3.iter_mut()

    v3.sort();
    println!("{:?} {:?} {:?}", v3, v3.len(), v3.capacity());
    
    v3.sort_by_key(|x| -x);
    println!("{:?} {:?} {:?}", v3, v3.len(), v3.capacity());
    
    v3.sort_unstable();
    println!("{:?} {:?} {:?}", v3, v3.len(), v3.capacity());
    
    v3.sort_unstable_by_key(|x| -x);
    println!("{:?} {:?} {:?}", v3, v3.len(), v3.capacity());
}