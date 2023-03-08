fn get_days_in_month(year: i32, month: i32) {
    // warunek roku przystępnego
    if((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0) {
        // jeśli miesiąc jest mniejszy od 8
        if month < 8 {
            // jeśli nie podzielny przez 2 to ma 31 dni
            if month % 2 != 0 {
                println!("Ten miesiąc ma 31 dni");
            // jeśli to luty to ma 29 dni
            } else if month == 2 {
                println!("Ten miesiąc ma 29 dni");
            // w przeciwnym przypadku ma 30 dni
            } else {
                println!("Ten miesiąc ma 30 dni");
            }
        // jeśli miesiąc jest większy równy 8
        } else {
            // jeśli podzielny przez 2 to ma 31 dni
            if month % 2 == 0 {
                println!("Ten miesiąc ma 31 dni");
            // w przeciwnym przypadku ma 30 dni
            } else {
                println!("Ten miesiąc ma 30 dni");
            }
        }
    // jeśli rok nie jest przystępny
    } else {
         // jeśli miesiąc jest mniejszy od 8
        if month < 8 {
            // jeśli nie podzielny przez 2 to ma 31 dni
            if month % 2 != 0 {
                println!("Ten miesiąc ma 31 dni");
            // jeśli to luty to ma 28 dni
            } else if month == 2 {
                println!("Ten miesiąc ma 28 dni");
            // w przeciwnym przypadku ma 30 dni
            } else {
                println!("Ten miesiąc ma 30 dni");
            }
        // jeśli miesiąc jest większy równy 8
        } else {
            // jeśli podzielny przez 2 to ma 31 dni
            if month % 2 == 0 {
                println!("Ten miesiąc ma 31 dni");
            // w przeciwnym przypadku ma 30 dni
            } else {
                println!("Ten miesiąc ma 30 dni");
            }
        }
    }
}

fn main() {
    let year = 2023;
    let month = 3;
    
    get_days_in_month(year, month);
}
