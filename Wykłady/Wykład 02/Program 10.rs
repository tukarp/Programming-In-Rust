fn main() {
    let mut a = 10;

    let b = if a % 2 == 0 {
        a / 2;
    } else {
        3 * a + 1;
    };

    println!("{:?}", b);
}
