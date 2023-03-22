// Funkcja zamieniająca dwa elementy tablicy
fn swap_array(array: &mut [i32; 10], i: usize, j: usize) {
    let temp = array[i];
    array[i] = array[j];
    array[j] = temp;
}

fn main() {
    // Utworzenie tablicy
    let mut array: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let index1 = 3;
    let index2 = 5;
    
    // Wyświetlenie tablicy przed zmianą
    println!("Tablica przed zmianą:");
    for i in 0..array.len() {
        println!("{}. Element tablicy: {}", i, array[i]);
    }
    println!();
    
    // Zamienienie elementów w tablicy
    println!("Zamieniamy liczby w tablicy o indeksach {} oraz {}", index1, index2);
    swap_array(&mut array, index1, index2);
    println!();
    
    // Wyświetlenie tablicy po zmianach
    println!("Tablica po zmianie:");
    for i in 0..array.len() {
        println!("{}. Element tablicy: {}", i, array[i]);
    }
}
