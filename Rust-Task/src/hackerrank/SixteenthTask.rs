use std::io::{self, BufRead};

fn divisible_sum_pairs(k: i32, ar: &[i32]) -> i32 {
    (0..ar.len())
        .flat_map(|i| (i + 1..ar.len()).map(move |j| (ar[i], ar[j])))
        .filter(|&(a, b)| (a + b) % k == 0)
        .count() as i32
}

pub fn divisible_sum_pairs_main() {
    println!("Divisible Sum Pairs");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let k = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = divisible_sum_pairs(k, &ar);

    print!("{}", result);
}