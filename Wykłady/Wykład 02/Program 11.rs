fn main() {
    let mut a = 10;

    a = if a % 2 == 0 {
        a / 2
    } else {
        let pom = 3*a;
        pom + 1
    };

    println!("{:?}", a);
}
