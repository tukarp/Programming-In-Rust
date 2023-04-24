fn greet(name: &str) {
    println!("Witaj, {}!", name);
}

fn greet2(name: String) {
    println!("Witaj, {}!", name);
}

fn main() {
    let name = "Edek";
    greet(name);
    let name2: String = "Felek".to_string();
    greet(&name2);
    greet(&name2);
    greet2(name2);
}

/*

przekazywanie danych do funkcji przez:
* wartość (czasem kopiowanie [typy kopiowalne, zwykle to typy proste], a czasem przeniesienie [inne])
* referencję (pożyczkę)
* referencję (pożyczkę) mutowalną

rozróżnienie pojęć: kopiowanie a klonowanie

*/