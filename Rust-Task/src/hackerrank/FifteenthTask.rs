use std::io::{self, BufRead};

fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    (0..=(s.len() - m as usize))
        .filter(|&i| s[i..i + m as usize].iter().sum::<i32>() == d)
        .count() as i32
}

pub fn subarray_division_main() {
    println!("Task 15. Subarray Division");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let s: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let d = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let result = birthday(&s, d, m);

    println!("{}", result);
}