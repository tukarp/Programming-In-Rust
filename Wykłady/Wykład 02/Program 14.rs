fn main() {
    let mut a = 10;
    let b = 20;

    let x = while a < b {
        a += 1;
        println!("{}", a);
    };
    
    println!("{:?}", x);
}
