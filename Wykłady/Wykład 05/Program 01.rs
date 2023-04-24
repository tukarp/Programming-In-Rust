fn main() {
    println!("============");
    for z in "katastrofa".chars().step_by(2) {
        println!("{}", z);
    }

    println!("============");
    for z in [1, 3, 4, 10] {
        println!("{}", z);
    }

    println!("============");
    for z in [1, 3, 4, 10].iter() {
        println!("{}", z);
    }

    println!("============");
    for z in (0..5).chain(50..55) {
        println!("{}", z);
    }

    println!("============");
    let vector: Vec<_> = (0..5).chain(50..55).collect();
    println!("{:?}", vector);

    println!("============");
    for z in (0..5).zip(50..55) {
        println!("{:?}", z);
    }

    println!("============");
    for z in (0..5).zip("buteleczka".chars()) {
        println!("{:?}", z);
        println!("{}", z.0);
        println!("{}", z.1);
    }

    println!("============");
    for z in "buteleczka".chars().enumerate() {
        println!("{:?}", z);
        println!("{}", z.0);
        println!("{}", z.1);
    }

    println!("============");
    for z in "buteleczka".chars().take(5) {
        println!("{:?}", z);
    }

    println!("============");
    let x = "buteleczka".chars().min();
    let y = "buteleczka".chars().max();
    println!("{:?}", x);
    println!("{:?}", y);

    println!("============");
    let x = "".chars().min();
    let y = "".chars().max();
    println!("{:?}", x);
    println!("{:?}", y);

    println!("============");
    let array: u8 = [1, 4, 36].iter().sum();
    println!("{:?}", array);
}
