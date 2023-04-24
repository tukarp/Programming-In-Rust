fn increment(x: i32) -> i32 {
    x + 1
}

fn increment(x: &mut i32) {
    *x += 1;
}

fn main() {
    let mut a = 12;
    
    let b = increment(a);
    println!("{}", b == 13);
    
    increment(&mut a);
    println!("{}", a == 13);
    increment(&mut a);
    println!("{}", a == 14);
}

/*

przekazywanie danych do funkcji przez:
* wartość (czasem kopiowanie [typy kopiowalne, zwykle to typy proste], a czasem przeniesienie [inne])
* referencję (pożyczkę)
* referencję (pożyczkę) mutowalną

rozróżnienie pojęć: kopiowanie a klonowanie

*/
