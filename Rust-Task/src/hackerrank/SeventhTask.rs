use std::io::{self, BufRead};

fn mini_max_sum(arr: &[i32]) {
    let total_sum: i32 = arr.iter().sum();
    let (min_value, max_value) = arr.iter().fold((i32::MAX, i32::MIN), |(min, max), &x| {
        (min.min(x), max.max(x))
    });
    
    let min_sum = total_sum - max_value;
    let max_sum = total_sum - min_value;
    
    println!("{} {}", min_sum, max_sum);
}

pub fn mini_max_sum_main() {
    println!("Task 7. Mini-Max Sum");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    println!("Введіть усі числа через пробіл: ");
    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap().trim_end().split(' ').map(|s| s.to_string().parse::<i32>().unwrap()).collect();
    
    mini_max_sum(&arr);
}