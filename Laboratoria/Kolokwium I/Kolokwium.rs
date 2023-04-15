// Kolokwium I

// Zadanie 1
fn zadanie(a: i64) -> bool {
    let a_float = a as f64;
    if(a_float < 2.5) && (a > 1) {
        true
    } else {
        false
    }
}

// Zadanie 2
fn moje_abs(x: i64) -> i64 {
    if x < 0 {
        x * (-1)
    } else {
        x
    }
}

// Zadanie 3
fn ocena(x: String) -> String {
    let mut wynik: i64 = 0;
    for c in x.chars() {
        println!("{}", c);
        if c == 'A' {
            wynik += 4;
        } else if c == 'K' {
            wynik += 3;
        } else if c == 'D' {
            wynik += 2;
        } else if c == 'W' {
            wynik += 1;
        }
    }
    let wynik_string = wynik.to_string();
    wynik_string
}

/*
// Zadanie 3 - niedokończone z <Option>
fn ocena_v2(x: String) -> Option<String> {
    let mut wynik: i64 = 0;
    let mut err = false;
    
    if x.len() == 13 {
        for c in x.chars() {
            println!("{}", c);
            if c == 'A' {
                wynik += 4;
            } else if c == 'K' {
                wynik += 3;
            } else if c == 'D' {
                wynik += 2;
            } else if c == 'W' {
                wynik += 1;
            } else if c == '1' && c.next == '0' {
                err = true;
                break;
            }
        }
    } else {
        err = true;
    }
    
    match err {
        false => Some(wynik as String),
        true => None
    }
}
*/

fn main() {
    // Zadanie 1
    let x1: i64 = 2;
    println!("{}", zadanie(x1));
    
    // Zadanie 2
    let x2: i64 = -5;
    println!("{}", moje_abs(x2));
    
    // Zadanie 3
    let x3 = String::from("AKDW109876543");
    println!("{}", ocena(x3));
    //println!("{}", ocena_v2(x3));
}
