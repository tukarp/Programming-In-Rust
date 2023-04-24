fn print_array(array: &[i32]) {
    println!("{:?}", array);
}

fn print_element(array: &[i32], i: usize) {
    let x = array.get(i);
    println!("{:?}, {:?}, {:?}", x, x.is_none(), x.unwrap_or(&999));
    println!("{:?}", x.unwrap());
}

fn main() {
    let array = [10, 20, 40, 100];
    print_array(&array);
    println!("{}", array[1]);
    array[2] = 567;
    println!("{}", array.len());
    print_element(&array, 1);
}
