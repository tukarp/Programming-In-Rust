fn main() {
    let mut a = 10;
    let b = 20;

    let x = loop {
        if a >= b {
            break;
        }
        a += 1;
        println!("{}", a);
    };

    println!("{:?}", x);
}
