fn dodaj_pisemnie(mut a: String, mut b: String) -> String {
    let mut wynik = String::new();
    
    if a.len() < b.len() {
        let temp = b;
        b = a;
        a = temp;
    }
    
    while a.len() > b.len() {
        b = format!("0{}", b)
    }
    
    let a_iterator = a.chars().rev();
    let b_iterator = b.chars().rev();
    let iterator = a_iterator.zip(b_iterator);
    let mut suma = 0;
    
    let mut przeniesienie = false;
    
    for pair in iterator {
        let zero = '0' as u32;
        let liczba_1 = pair.0 as u32 - zero;
        let liczba_2 = pair.1 as u32 - zero;
        let mut suma = liczba_1 + liczba_2;
        
        if suma > 9 {
            przeniesienie = true;
            suma -= 10;
        } else {
            przeniesienie = false;
        }
    }
    
    wynik = format!("{}{}", suma, wynik);
    wynik
}

fn wizytowka_option(imie: String, nazwisko: String) -> Result<String, String> {
    let mut result = String::new();
    
    if !imie.chars().nth(0).unwrap().is_ascii_alphabetic() {
        return Err("Blad w imieniu".to_string());
    }
    
    result.push(imie.chars().nth(0).unwrap().to_ascii_uppercase());
    result.push_str(". ");
    
    if !nazwisko.chars().nth(0).unwrap().is_ascii_alphabetic() {
        return Err("Blad w nazwisku".to_string());
    }
    
    result.push(nazwisko.chars().nth(0).unwrap().to_ascii_lowercase());
    for c in nazwisko.chars().skip(1) {
        if !c.is_ascii_alphabetic() {
            return Err("Blad w nazwisku".to_string());
        }
        result.push(c.to_ascii_lowercase());
    }
    
    Ok(result)
}

fn znajdz_indeks(znak: char, tablica: &[char]) -> Option<usize> {
    let mut i = 0;
    let mut err = true;
    
    for z in tablica {
        if *z == znak {
            err = false;
            break;
        }
        i += 1;
    }
    
    match err {
        false => Some(i as usize),
        true => None
    }
}

fn z_rzymskich(napis: String) -> Result<u32, usize> {
    let mut iter = napis.chars();
    let mut wynik : i64 = 0;
    let rzymskie = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];
    let wartosc = [1, 5, 10, 50, 100, 500, 1000];
    let mut poprzedni = iter.nth(0).unwrap();
    let mut numer_znaku: usize = 0;
    
    let temp = znajdz_indeks(poprzedni, &rzymskie);
    match temp {
        Some(indeks) => wynik += wartosc[indeks],
        None => return Err(numer_znaku)
    }
    &
    numer_znaku += 1;
    for znak in iter {
        let poprzedni_indeks: usize;
        let temp = znajdz_indeks(poprzedni, &rzymskie);
        match temp {
            Some(indeks) => poprzedni_indeks = indeks,
            None => return Err(numer_znaku)
        }
        
        let obecny_indeks: usize;
        let temp = znajdz_indeks(znak, &rzymskie);
        match temp {
            Some(indeks) => obecny_indeks = indeks,
            None => return Err(numer_znaku)
        }
        
        if poprzedni_indeks < obecny_indeks {
            wynik -= 2 * wartosc[poprzedni_indeks];
        }
        poprzedni = znak;
        wynik += wartosc[obecny_indeks];
        numer_znaku += 1;
    }
    Ok(wynik as u32)
}

fn usun_znak(napis: &String, indeks: usize) -> String {
    let mut wynik = String::new();
    let mut i = 0;&
    for znak in napis.chars() {
        if i == indeks {
            continue;
        }
        wynik.push(znak);
        i += 1;
    }
    wynik
}

fn bezpieczne_z_rzymskich(wejscie: String) -> u32 {
    let mut napis = wejscie;
    loop {
        let wynik = z_rzymskich(&napis);
        match wynik {
            Ok(x) => return x,
            Err(indeks) => napis = usun_znak(&napis, indeks)
        }
    }
}

fn main() {
    //println!("{}", wizytowka_option("Tomasz".to_string(), "Wnuk!".to_string()));
    //println!("{}", dodaj_pisemnie("4".to_string(), "24".to_string()));
}
