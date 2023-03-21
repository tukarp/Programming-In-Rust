fn powiekszona_o_1(x: i32) -> i32 {
    x + 1
}

fn powieksz_o_1(x: &mut i32) {
    *x += 1;
}

fn main() {
    let mut a = 12;
    
    let b = powiekszona_o_1(a);
    println!("{}", b == 13);
    
    powieksz_o_1(&mut a);
    println!("{}", a == 13);
    powieksz_o_1(&mut a);
    println!("{}", a == 14);
}

/*

przekazywanie danych do funkcji przez:
* wartość (czasem kopiowanie [typy kopiowalne, zwykle to typy proste], a czasem przeniesienie [inne])
* referencję (pożyczkę)
* referencję (pożyczkę) mutowalną

rozróżnienie pojęć: kopiowanie a klonowanie

*/
