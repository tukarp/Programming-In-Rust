fn srednia(tab: &[f64]) -> Result<f64, String> {
    if tab.len() == 0 {
        Err(String::from("pusta tablica!"))
    } else {
        let s: f64 = tab.iter().sum();
        Ok(s / (tab.len() as f64))
    }
}

fn ile_powyzej_sredniej(tab: &[f64]) -> Option<usize> Result<usize, String> {
    let mut ile = 0;
    let sr = srednia(&tab)?;
//  let sr = srednia(&tab);
//  if sr.is_err() {
//      return Err(sr.err().unwrap());    // return Err(String::from("pusta tablica!"));
//  }
//  let sr = sr.unwrap();
    for x in tab {
        if * x > sr {
            ile += 1;
        }
    }
    Ok(ile)
}

fn main() {
    println!("{:?}", srednia(&[1.0, 3.0]));
    println!("{:?}", srednia(&[1.0, 3.0, 4.5]));
    println!("{:?}", srednia(&[]));

    println!("{:?}", ile_powyzej_sredniej(&[1.0, 3.0]));
    println!("{:?}", ile_powyzej_sredniej(&[1.0, 3.0, 4.5]));
    println!("{:?}", ile_powyzej_sredniej(&[]));
}
