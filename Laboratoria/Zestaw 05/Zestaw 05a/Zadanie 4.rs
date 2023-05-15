// Funkcja zwracająca wektor indeksów na których miejscu znajduję sie podany element
fn indexes(vector: &Vec<usize>, element: usize) -> Vec<usize> {
    let mut result = Vec::new();
    
    for i in 0..vector.len() {
        if vector[i] == element {
            result.push(i);
        }
    }
    result
}

fn main() {
    println!("{:?}", indexes(&vec![1, 2, 1, 3], 1));
}