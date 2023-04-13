use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;

// Funkcja generująca liczby pseudolosowe
fn rand(seed: &mut u64, min_random: u64, max_random: u64) -> u64 {
    let mut rng = StdRng::seed_from_u64(*seed);
    let random_number = rng.gen_range(min_random..max_random);
    return random_number;
}

fn main() {
    let mut seed: u64 = 1248;
    let min_random: u64 = 0;
    let max_random: u64 = 100;
    let random_number = rand(&mut seed, min_random, max_random);
    
    println!("Losowo wygenerowana liczba z ziarnem {} to: {}", seed, random_number);
}
