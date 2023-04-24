fn main() {
    let mut a = 10;

    a = if a % 2 == 0 {
        a / 2
    } else {
        let temp = 3 * a;
        temp + 1
    };

    println!("{:?}", a);
}
