// Funkcja zwracająca wektor alfabetu
fn exercise_1() -> Vec<char> {
    let vector: Vec<char> = ('a'..'z').collect();
    vector
}

// Funkcja zwracająca wektor kwadratów 10 kolejnych liczb całkowitych
fn exercise_2() -> Vec<i32> {
   let vector: Vec<i32> = (1..=10).map(|x| x * x).collect();
   vector
}

// Funkcja zwracająca wektor 10 kolejnych potęg dwójki
fn exercise_3() -> Vec<i32> {
   let vector: Vec<i32> = (1..=10).map(|x| i32::pow(2, x)).collect();
   vector
}

// Funkcja zwracająca wektor odwrotności liczb od 1 do 20
fn exercise_4() -> Vec<f32> {
   let mut vector: Vec<f32> = Vec::new();
   for i in 1..20 {
       let j = i as f32;
       vector.push(1.0 / j);
   }
   vector
}

// Funkcja zwracająca wektor liczby od 1 do 100 podzielne przez 3, ale niepodzielne przez 4
fn exercise_5() -> Vec<i32> {
   let mut vector: Vec<i32> = Vec::new();
   for i in 1..100 {
       if i % 3 == 0 && i % 4 != 0 {
           vector.push(i);
       }
   }
   vector
}

fn main() {
   // Zadanie 5.1
   println!("Alfabet:");
   println!("{:?}", exercise_1());
   
   // Zadanie 5.2
   println!("Kwadraty liczb:");
   println!("{:?}", exercise_2());
   
   // Zadanie 5.3
   println!("Potęgi dwójki:");
   println!("{:?}", exercise_3());
   
   // Zadanie 5.4
   println!("Odwrotności liczb:");
   println!("{:?}", exercise_4());
   
   // Zadanie 5.5
   println!("Liczby podzielne przez 3, ale niepodzielne przez 4:");
   println!("{:?}", exercise_5());
}