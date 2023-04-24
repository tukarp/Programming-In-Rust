fn swap(a: &mut i64, b: &mut i64) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn main() {
    let mut x = 10;
    let mut y = 200;
    swap(&mut x, &mut y);
    println!("{}, {}", x, y);
    
    let a = [100, 20, 40];
}
