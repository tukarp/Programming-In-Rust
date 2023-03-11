fn leap_year_check(year: i32) {
    if((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0) {
        println!("This year is leap!");
    } else {
        println!("This year isn't leap!");
    }
}

fn main() {
    let year = 2023;
    leap_year_check(year);
}
