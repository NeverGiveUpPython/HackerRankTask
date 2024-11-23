use std::io::{self, BufRead};

fn simple_array_sum(ar: &[i32]) -> i32 {
    ar.iter().sum()
}

pub fn simple_array_sum_main() {
    println!("Task 1. Simple-Array-Sum");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    println!("Введіть кількість елементів масиву: ");
    let _ar_count: i32= stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    println!("Введіть всі числа масиву: ");
    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap().trim_end().split(' ').map(|s: &str| s.to_string().parse::<i32>().unwrap()).collect();

    let result: i32 = simple_array_sum(&ar);

    println!("Cума всіх чисел дорівнює: {}", result);
}