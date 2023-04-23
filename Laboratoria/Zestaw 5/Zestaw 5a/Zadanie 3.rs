// Funkcja zwracająca ilość powtarzających się wartości w dwóch wektorach
fn count_repeats(vector1: &Vec<i32>, vector2: &Vec<i32>) -> u8 {
    let mut result = 0;
    
    for value1 in vector1 {
        for value2 in vector2 {
            if value1 == value2 {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    println!("{:?}", count_repeats(&vec![1, 2, 1, 3], &vec![1, 2]));
}