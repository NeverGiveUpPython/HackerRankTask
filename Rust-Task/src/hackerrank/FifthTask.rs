use std::io::{self, BufRead};

fn plus_minus(arr: &[i32]) {
    let n = arr.len() as f64; // Довжина масиву
    
    let mut positive_count = 0i8;
    let mut negative_count = 0i8;
    let mut zero_count = 0i8;

    for &num in arr {
        if num > 0 {
            positive_count += 1;
        } else if num < 0 {
            negative_count += 1;
        } else {
            zero_count += 1;
        }
    }

    let positive_proportion = positive_count as f64 / n;
    let negative_proportion = negative_count as f64 / n;
    let zero_proportion = zero_count as f64 / n;

    println!("{:.6}", positive_proportion);
    println!("{:.6}", negative_proportion);
    println!("{:.6}", zero_proportion);
}

pub fn plus_minus_main() {
    println!("Task5. Plus-Minus");
    let stdin = io::stdin();

    let mut stdin_iterator = stdin.lock().lines();

    println!("Введіть усі числа через пробіл:");
    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap().trim_end().split(' ').map(|s| s.to_string().parse::<i32>().unwrap()).collect();

    plus_minus(&arr);
}