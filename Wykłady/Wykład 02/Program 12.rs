fn nwd(a : i32, b : i32) -> i32 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    if b > a {
        return nwd(b%a, a);
    }
    return nwd(a%b, b);
}

fn main() {
    println!("{}", nwd(100, 30));
}
