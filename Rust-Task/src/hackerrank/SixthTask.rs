use std::io::{self, BufRead};

fn staircase(n: i32) {
    for i in 1..=n {
        let spaces = " ".repeat((n - i) as usize);
        let hashes = "#".repeat(i as usize);
        
        println!("{}{}", spaces, hashes);
    }
}

pub fn staircase_main() {
    println!("Task 6. Staircase");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    println!("Введіть розмір сходів: ");
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    staircase(n);
}