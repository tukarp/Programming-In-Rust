fn main() {
    let v: Vec<_> = (1..=10).map(|n| n * n).collect();
    println!("{:?}", v);
    let v: Vec<_> = (1..=10).map(|n| n + 9).collect();
    println!("{:?}", v);
    let k = 9;
    let v: Vec<_> = (1..=10).map(|n| n + k).collect();
    println!("{:?}", v);
    let v: Vec<_> = (1..=100).filter(|n| n % 10 == 1).collect();
    println!("{:?}", v);
    let x: i32 = (1..=100).sum();
    println!("{:?}", x);
    let x: Option<i32> = (1..=100).reduce(|acc, x| acc + x);
    println!("{:?}", x);
    let x: Option<i32> = (1..=100).filter(|n| n > &100000).reduce(|acc, x| acc + x);
    println!("{:?}", x);
    let x: i32 = (1..=100).fold(0, |acc, x| acc + x);
    println!("{:?}", x);
    let x = (16..=100).find(|n| n % 6 == 0 && n % 15 == 0);
    println!("{:?}", x);
    let x = (16..=100).rfind(|n| n % 6 == 0 && n % 15 == 0);
    println!("{:?}", x);

    let v: Option<_> = (1..=100).find(|n| n % 10 != 5);
    println!("{:?}", v);
    let v: Vec<_> = (1..=100).filter(|n| n % 10 != 5).collect();
    println!("{:?}", v);
    let v: Vec<_> = (1..=100).take_while(|n| n % 10 != 5).collect();
    println!("{:?}", v);
    
    let v: Vec<_> = (1..=100).skip_while(|n| n % 10 != 5).collect();
    println!("{:?}", v);
    
    let x = (1..).map(|x| 1.0/(x as f64)).find(|x| x < &0.03);
    println!("{:?}", x);
    
    let x = (1..).map(|x| 1.0/(x as f64)).enumerate().find(|x| x.1 < 0.03);
    println!("{:?}", x);

    let x = (0..).map(|x| 1.0/(x as f64)).enumerate().find(|x| x.1 < 0.03);
    println!("{:?}", x);
    
    let x = (10..20).all(|x| x > 0);
    println!("{:?}", x);
    let x = (10..20).any(|x| x > 15);
    println!("{:?}", x);
    let x = (10..).all(|x| x < 20);
    println!("{:?}", x);
    let x = (10..).any(|x| x < 20);
    println!("{:?}", x);
    
    let v: Vec<_> = (1..10).map(|x| println!("{:?}", x)).collect();
    println!("{:?}", v);
    
    (1..10).map(|x| println!("{:?}", x)).collect::<Vec<_>>();
    
    (1..10).for_each(|x| println!("{:?}", x));
}