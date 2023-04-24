#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

fn main() {
    let mut point1 = Point3D {
        x: 4.5,
        z: -7.5,
        y: 3.0,
    };
    println!("{:?}", point1);
    let point22 = Point3D {
        z: 47.5,
        ..point1
    };
    println!("{:?}", point2);
    point1.x = 34.12;
    println!("{:?}", point1);
    println!("{:?}", point1 == point2);
    println!("{:?}", point1 < point2);
    println!("{:?}", point1 > point2);
    
    let point3 = point1.clone();
    let point4 = point1.clone();
    println!("{:?}", point1);
    println!("{:?}", point3);
    println!("{:?}", point4);
    println!("{:?}", point1 == point4);
}