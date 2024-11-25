use std::io::{self, BufRead};

fn bon_appetit(bill: &[i32], k: i32, b: i32) {
    let total_sum: i32 = bill.iter()
    .enumerate()
    .filter(|&(i, _)| i as i32 != k)
    .map(|(_, &price)| price)
    .sum();

let fair_share = total_sum / 2;

match b == fair_share {
    true => println!("Bon Appetit"),
    false => println!("{}", b - fair_share),
    }
}

pub fn bill_division_main() {
    println!("Task 18. Bill Division");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let bill: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let b = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    bon_appetit(&bill, k, b);
}
