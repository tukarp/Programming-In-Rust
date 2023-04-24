fn square_up(x: i32) -> i32 {
    x * x
}

fn main() {
    let x = 0..;

    let x = (1..=10).map(square_up);

    let vector: Vec<_> = (1..=10).map(square_up).collect();
    println!("{:?}", vector);

    let vector: Vec<_> = (1..=10).map(|x| x * x).collect();
    println!("{:?}", vector);
}