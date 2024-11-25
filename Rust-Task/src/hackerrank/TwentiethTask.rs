use std::io::{self, BufRead};



fn page_count(n: i32, p: i32) -> i32 {
    let from_front = p / 2;
    let from_back = (n - p + 1) / 2;

    from_front.min(from_back)
}

pub fn drawing_book_main() {
    println!("Task 20. Drawing Book");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let p = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = page_count(n, p);

    println!("{}", result);
}