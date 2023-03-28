fn wyswietl(t: &[i32]) {
    println!("{:?}", t);
}

fn main() {
    let mut tablica = [10, 20, 40, 100];
    wyswietl(&tablica);
    println!("{}", tablica[1]);
    tablica[2] = 567;
//  tablica[100] = 987;     // błąd -- na poziomie kompilacji
}
