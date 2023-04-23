// Funkcja zwracająca powtarzające się wartości w wektorze
fn repeats(vector: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    
    if vector.len() == 2 {
        let mut a = vector.get(0).unwrap();
        let mut b = vector.get(1).unwrap();
        if a == b {
            result.push(*a);
            result.push(*b);
        }
    }  else if vector.len() > 2 {
        let mut a = vector.get(0).unwrap();
        let mut b = vector.get(1).unwrap();
        let mut c = vector.get(3).unwrap();
        let mut i: usize = 3;
        
        if a == b {
            result.push(*a);
            result.push(*b);
        } while i < vector.len() {
            if a == b || b == c {
                result.push(*b);
            }
            a = b;
            b = c;
            match vector.get(i) {
                Some(x) => c = x,
                None => c = &0
                
            }
            i += 1;
        }
        if a == b {
            result.push(*b);
        }
    }
}

fn main() {
    println!("{:?}", repeats(&vec![1, 3, 4, 3, 3, 3, 3, 4, 1, 1, 6]));
}