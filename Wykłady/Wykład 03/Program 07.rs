fn powitaj(imie: &str) {
    println!("Witaj, {}!", imie);
}

fn powitaj_z_przejeciem(imie: String) {
    println!("Witaj, {}!", imie);
}

fn main() {
    let imie = "Edek";
    powitaj(imie);
    let inne_imie: String = "Felek".to_string();
    powitaj(&inne_imie);
    powitaj(&inne_imie);
    powitaj_z_przejeciem(inne_imie);
//     powitaj_z_przejeciem(inne_imie); // tu już nie działa
}

/*

przekazywanie danych do funkcji przez:
* wartość (czasem kopiowanie [typy kopiowalne, zwykle to typy proste], a czasem przeniesienie [inne])
* referencję (pożyczkę)
* referencję (pożyczkę) mutowalną

rozróżnienie pojęć: kopiowanie a klonowanie

*/
