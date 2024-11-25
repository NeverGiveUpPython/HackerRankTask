use std::io::{self, BufRead};
use std::collections::HashMap;

fn sock_merchant(ar: &[i32]) -> i32 {
    let mut counts = HashMap::new();

    // Підрахунок кількості кожного типу шкарпеток
    for &sock in ar {
        *counts.entry(sock).or_insert(0) += 1;
    }

    // Підрахунок пар
    counts.values().map(|&count| count / 2).sum()
}

pub fn sales_by_match_main() {
    println!("Task 19. Sales by Match");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = sock_merchant(&ar);

    println!("{}", result);
}
