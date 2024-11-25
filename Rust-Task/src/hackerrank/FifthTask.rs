use std::io::{self, BufRead};

fn plus_minus(arr: &[i32]) {
    let n = arr.len() as f64; // Довжина масиву

    let (positive_count, negative_count, zero_count) = arr.iter().fold((0, 0, 0), |(pos, neg, zero), &num| {
        if num > 0 {
            (pos + 1, neg, zero)
        } else if num < 0 {
            (pos, neg + 1, zero)
        } else {
            (pos, neg, zero + 1)
        }
    });

    let positive_proportion = positive_count as f64 / n;
    let negative_proportion = negative_count as f64 / n;
    let zero_proportion = zero_count as f64 / n;

    println!("{:.6}", positive_proportion);
    println!("{:.6}", negative_proportion);
    println!("{:.6}", zero_proportion);
}

pub fn plus_minus_main() {
    println!("Task 5. Plus-Minus");
    let stdin = io::stdin();

    let mut stdin_iterator = stdin.lock().lines();

    println!("Введіть усі числа через пробіл:");
    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap().trim_end().split(' ').map(|s| s.to_string().parse::<i32>().unwrap()).collect();

    plus_minus(&arr);
}