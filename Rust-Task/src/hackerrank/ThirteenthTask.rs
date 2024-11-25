use std::io::{self, BufRead};
fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let start = *a.iter().max().unwrap();
    let end = *b.iter().min().unwrap();

    (start..=end).filter(|&x| {
        a.iter().all(|&ai| x % ai == 0) && b.iter().all(|&bi| bi % x == 0)
    }).count() as i32
}

pub fn between_two_sets_main() {
    println!("Task 13. Between Two Sets");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap().trim_end().split(' ').map(|s| s.to_string().parse::<i32>().unwrap()).collect();

    let brr: Vec<i32> = stdin_iterator.next().unwrap().unwrap().trim_end().split(' ').map(|s| s.to_string().parse::<i32>().unwrap()).collect();

    let total = get_total_x(&arr, &brr);

    println!("{}", total);
}