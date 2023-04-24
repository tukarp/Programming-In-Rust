fn greatest_common_divisor(a : i32, b : i32) -> i32 {
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else if b > a {
        greatest_common_divisor(b %a , a)
    } else {
        greatest_common_divisor(a % b, b)
    }
}

fn main() {
    println!("{}", greatest_common_divisor(100, 30));
}
