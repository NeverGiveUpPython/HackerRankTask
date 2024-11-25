use std::io::{self, BufRead};

fn time_conversion(s: &str) -> String {
    let hour: i32 = s[0..2].parse().unwrap();
    let minutes_and_seconds = &s[2..8];
    let period = &s[8..].to_lowercase(); // перетворюємо на нижній регістр для спрощення

    let converted_hour = match period.as_str() {
        "am" if hour == 12 => 0,
        "pm" if hour != 12 => hour + 12,_ => hour,
    };

format!("{:02}{}", converted_hour, minutes_and_seconds)
}

pub fn time_conversion_main() {
    println!("Task 9. Time Conversion");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    println!("Впишіть час 12-годиному форматі часу: ");
    let s = stdin_iterator.next().unwrap().unwrap();

    let result = time_conversion(&s);

    println!("В 24-годиний формат часу: {}", result);
}
