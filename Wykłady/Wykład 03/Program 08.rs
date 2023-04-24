fn print_array(array: &[i32]) {
    println!("{:?}", array);
}

fn main() {
    let array = [1, 2, 4, 10];
    print_array(&array);
}
