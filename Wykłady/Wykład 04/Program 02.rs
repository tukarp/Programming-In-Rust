fn wyswietl(t: &[i32]) {
    println!("{:?}", t);
}

fn wyswietl_jeden(t: &[i32], i: usize) {
    println!("{:?}", t[i]);
}

fn main() {
    let mut tablica = [10, 20, 40, 100];
    wyswietl(&tablica);
    println!("{}", tablica[1]);
    tablica[2] = 567;
//     wyswietl_jeden(&tablica, 100);  // błąd -- w czasie wykonania
}
