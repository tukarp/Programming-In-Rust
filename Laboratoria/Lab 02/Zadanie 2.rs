// Sprawdź czy rok jest przystępny
fn leap_year_check(year: i32) -> bool {
    let leap_year;
    if((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0) {
        leap_year = true;
        println!("This year is leap!");
    } else {
        leap_year = false;
        println!("This year isn't leap!");
    }
    return leap_year;
}

// Sprawdź ile dni ma dany miesiąc
fn get_month_days(year: i32, month: i32) {
    // Miesiące które mają 31 dni
    if month == 1 || month == 3 || month == 5 || month == 7 || month == 8 || month == 10 || month == 12 {
        println!("{month} month has 31 days!");
    // Luty
    } else if month == 2 {
        // Jeśli rok jest przystępny
        if leap_year_check(year) {
            println!("{month} month has 29 days!");
        // W przeciwnym przypadku
        } else {
            println!("{month} month has 28 days!");
        }
    // Miesiące które mają 30 dni
    } else {
        println!("{month} month has 30 days");
    }
}

fn main() {
    let year = 2023;
    let month = 3;
    get_month_days(year, month);
}
