#[derive(Debug, PartialEq, Clone)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    
    fn on_x_axis(x: f64) -> Point3D {
        Point3D {
            x: x,
            ..Punkt3D::default()
        }
    }
    
    fn default() -> Point3D {
        Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

fn main() {
    let mut point1 = Point3D {
        x: 4.5,
        z: -7.5,
        y: 3.0,
    };
    println!("{:?}", point1);
    let point2 = Point3D {
        z: 47.5,
        ..point1
    };
    println!("{:?}", point2);
    point1.x = 34.12;
    
    println!("{:?}", point1.norm());
    println!("{:?}", Point3D::norm(&point1));
    println!("{:?}", point1);
    
    let point3 = Point3D::on_x_axis(3.1);
    println!("{:?}", point3);
    println!("{:?}", point3.norm());
    
    let vector = vec![
        None,
        Some(point1.clone()),
        Some(point3.clone()),
        None,
        Some(Point3D::on_x_axis(-9.0)),
    ];
    println!("{:?}", vector);
    for point in &vector {
        println!("{:?}", point);
    }
    for point in &vector {
        println!("{:?}", point.clone().unwrap_or_default());
    }
}