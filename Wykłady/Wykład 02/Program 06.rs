fn main() {
    let mut a = 10;
    let b = 20;

    while a < b {
        a += 1;
        if a == 15 {
            continue;
        }
        if a == 17 {
            break;
        }
        println!("{}", a);
    }
}
