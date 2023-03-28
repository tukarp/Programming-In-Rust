fn srednia(tab: &[f64]) -> f64 {
    let s: f64 = tab.iter().sum();
    s / (tab.len() as f64)
}

fn main() {
    println!("{:?}", srednia(&[1.0, 3.0]));
    println!("{:?}", srednia(&[1.0, 3.0, 4.5]));
    println!("{:?}", srednia(&[]));
}
