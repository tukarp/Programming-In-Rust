fn srednia(tab: &[f64]) -> Option<f64> {
    if tab.len() == 0 {
        None
    } else {
        let s: f64 = tab.iter().sum();
        Some(s / (tab.len() as f64))
    }
}

fn ile_powyzej_sredniej(tab: &[f64]) -> Option<usize> {
    let mut ile = 0;
    let sr = srednia(&tab)?;
//  let sr = srednia(&tab);
//  if sr.is_none() {
//      return None;
//  }
//  let sr = sr.unwrap();
    for x in tab {
        if *x > sr {
            ile += 1;
        }
    }
    Some(ile)
}

fn main() {
    println!("{:?}", srednia(&[1.0, 3.0]));
    println!("{:?}", srednia(&[1.0, 3.0, 4.5]));
    println!("{:?}", srednia(&[]));

    println!("{:?}", ile_powyzej_sredniej(&[1.0, 3.0]));
    println!("{:?}", ile_powyzej_sredniej(&[1.0, 3.0, 4.5]));
    println!("{:?}", ile_powyzej_sredniej(&[]));
}
