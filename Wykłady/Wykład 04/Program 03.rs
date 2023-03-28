fn wyswietl(t: &[i32]) {
    println!("{:?}", t);
}

fn wyswietl_jeden(t: &[i32], i: usize) {
    let x = t.get(i);
    println!("{:?}, {:?}, {:?}", x, x.is_none(), x.unwrap_or(&999));
    println!("{:?}", x.unwrap());
}

fn main() {
    let mut tablica = [10, 20, 40, 100];
    wyswietl(&tablica);
    println!("{}", tablica[1]);
    tablica[2] = 567;
    println!("{}", tablica.len());
    wyswietl_jeden(&tablica, 1);
//  wyswietl_jeden(&tablica, 100);      // błąd - w czasie wykonania
}
