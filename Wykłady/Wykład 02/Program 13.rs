fn nwd(a : i32, b : i32) -> i32 {
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else if b > a {
        nwd(b%a, a)
    } else {
        nwd(a%b, b)
    }
}

fn main() {
    println!("{}", nwd(100, 30));
}
