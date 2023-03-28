fn swap(a: &mut i64, b: &mut i64) {
    let pom = *a;
    *a = *b;
    *b = pom;
}

fn main() {
    let mut x = 10;
    let mut y = 200;
    swap(&mut x, &mut y);
//     swap(&mut x, &mut x);    // nie działa ze względu na dwie pożyczki mutowalne z tego samego źródła
    println!("{}, {}", x, y);
    
    let a = [100, 20, 40];
//     swap(&mut a[1], &mut a[2]);  // nie działa ze względu na dwie pożyczki mutowalne z tego samego źródła
//     swap_arr(&mut a, 1, 2);  // będize ok
}
