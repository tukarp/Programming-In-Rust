fn average(array: &[f64]) -> f64 {
    let x: f64 = array.iter().sum();
    x / (array.len() as f64)
}

fn main() {
    println!("{:?}", average(&[1.0, 3.0]));
    println!("{:?}", average(&[1.0, 3.0, 4.5]));
    println!("{:?}", average(&[]));
}
