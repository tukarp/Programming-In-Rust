fn print_array(array: &[i32]) {
    println!("{:?}", array);
}

fn main() {
    let array = [10, 20, 40, 100];
    print_array(&array);
    println!("{}", array[1]);
    array[2] = 567;
}
