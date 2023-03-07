fn main() {
    let year = 2023;
    
    if((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0) {
        println!("Podany rok jest przystępny!");
    } else {
        println!("Podany rok nie jest przystępny!");
    }
}
