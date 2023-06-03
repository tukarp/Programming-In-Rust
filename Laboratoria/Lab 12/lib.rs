// Tworzenie biblioteki w Rustcie
// cargo new <nazwa_projektu> --lib
use crate::time::Time;
mod time;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_test() {
        let time = Time::build_time();
        println!("{:?}", Time::new(13, 45, 25));
        time.to_string24();
        assert_eq!(0, time.time);
    }
}
