fn average(array: &[f64]) -> Result<f64, String> {
    if array.len() == 0 {
        Err(String::from("Empty array!"))
    } else {
        let x: f64 = array.iter().sum();
        Ok(x / (array.len() as f64))
    }
}

fn how_much_more_than_average(array: &[f64]) -> Option<usize> Result<usize, String>  {
    let mut counter = 0;
    let average = average(&array);

    for x in array {
        if *x > average {
            counter += 1;
        }
    }
    Ok(counter)
}

fn main() {
    println!("{:?}", average(&[1.0, 3.0]));
    println!("{:?}", average(&[1.0, 3.0, 4.5]));
    println!("{:?}", average(&[]));

    println!("{:?}", how_much_more_than_averagej(&[1.0, 3.0]));
    println!("{:?}", how_much_more_than_average(&[1.0, 3.0, 4.5]));
    println!("{:?}", how_much_more_than_average(&[]));
}
