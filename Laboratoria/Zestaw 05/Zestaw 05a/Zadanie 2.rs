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

// Funkcja zwracająca wektor unikalnych wartości z oryginalnego wektora
fn unique_values(vector: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    
    for value in vector {
        if count_repeats(vector, &vec![*value]) == 1 {
            result.push(*value)
        }
    }
    result
}

fn main() {
    println!("{:?}", unique_values(&vec![1, 3, 4, 3, 3, 3, 3, 4, 1, 1, 6]));
}