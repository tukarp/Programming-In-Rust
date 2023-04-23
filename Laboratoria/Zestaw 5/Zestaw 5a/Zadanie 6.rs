// Funkcja zwracająca napisy krótsze niż 4 znaki z podanego wektora
fn exercise_1(vector: &Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    
    for string in vector {
        if string.len() < 4 {
            result.push(string.to_string());
        }
    }
    result
}

// Funkcja zwracająca napisy nie zawierające 'A' i 'a' z podanego wektora
fn exercise_2(vector: &Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    let mut condition = false;
    
    for string in vector {
        for char in string.chars() {
            if char == 'A' || char == 'a' {
                condition = true;
                break;
            }
        } if condition == false {
            result.push(string.to_string());
        }
        condition = false;
    }
    result
}

// Funkcja zwracająca wektor napisów zawierających cyfry z podanego wektora
fn exercise_3(vector: &Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    let mut condition = false;
    
    for string in vector {
        for char in string.chars() {
            if char.is_ascii_digit() {
                condition = true;
                break;
            }
        } if condition == false {
            result.push(string.to_string());
        }
        condition = false;
    }
    result
}


// Funkcja zwracająca wektor odwróconych napisów z podanego wektora
fn exercise_4(vector: &Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    
    for string in vector {
        result.push(string.chars().rev().collect::<String>())
    }
    result
}


// Funkcja zwracająca wektor napisów z podwojonymi literami z podanego wektora
fn exercise_5(vector: &Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    
    for string in vector {
        let mut temp_string = String::new();
        for char in string.chars() {
            temp_string.push(char);
            temp_string.push(char);
        }
        result.push(temp_string)
    }
    result
}


fn main() {
    // Zadanie 6.1
    println!("Wektor napisów krótszych niż 4 znaki:");
    println!("{:?}", exercise_1(&vec!["abc".to_string(), "defg".to_string(), "123".to_string(), "456789".to_string()]));
    
    // Zadanie 6.2
    println!("Wektor napisów niezawierający liter 'A' ani 'a':");
    println!("{:?}", exercise_2(&vec!["abc".to_string(), "defg".to_string(), "123".to_string(), "456789A".to_string()]));
    
    // Zadanie 6.3
    println!("Wektor napisów zawierających cyfry:");
    println!("{:?}", exercise_3(&vec!["abc".to_string(), "defg".to_string(), "123".to_string(), "456789A".to_string()]));
    
    // Zadanie 6.4
    println!("Wektor odwróconych napisów:");
    println!("{:?}", exercise_4(&vec!["abc".to_string(), "defg".to_string(), "123".to_string(), "456789A".to_string()]));
    
    // Zadanie 6.5
    println!("Wektor napisów z podwojonymi litererami:");
    println!("{:?}", exercise_5(&vec!["abc".to_string(), "defg".to_string(), "123".to_string(), "456789A".to_string()]));
}