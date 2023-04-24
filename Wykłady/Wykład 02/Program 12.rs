fn greatest_common_divisor(a : i32, b : i32) -> i32 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    if b > a {
        return greatest_common_divisor(b % a, a);
    }
    return greatest_common_divisor(a % b, b);
}

fn main() {
    println!("{}", greatest_common_divisor(100, 30));
}
