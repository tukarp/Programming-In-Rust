fn average(array: &[f64]) -> Option<f64> {
    if array.len() == 0 {
        None
    } else {
        let x: f64 = array.iter().sum();
        Some(x / (array.len() as f64))
    }
}

fn how_much_more_than_average(array: &[f64]) -> Option<usize> {
    let mut counter = 0;
    let average = average(&array);
    if average.is_none() {
        return None;
    }
    let average = average.unwrap();
    for x in array {
        if *x > average {
            counter += 1;
        }
    }
    Some(counter)
}

fn main() {
    println!("{:?}", average(&[1.0, 3.0]));
    println!("{:?}", average(&[1.0, 3.0, 4.5]));
    println!("{:?}", average(&[]));

    println!("{:?}", how_much_more_than_averagej(&[1.0, 3.0]));
    println!("{:?}", how_much_more_than_average(&[1.0, 3.0, 4.5]));
    println!("{:?}", how_much_more_than_average(&[]));
}
