use std::io::{self, BufRead};

fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apples_count = apples.iter().filter(|&&x| x + a >= s && x + a <= t).count();
    let oranges_count = oranges.iter().filter(|&&x| x + b >= s && x + b <= t).count();
    
    println!("{}", apples_count);
    println!("{}", oranges_count);
}
pub fn apple_and_orange_main() {
    println!("Task11. Apple and Orange");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    println!("Введіть кординати місцезнаходження будинку Сема, початок та кінець через пробіл: ");
    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap().split(' ').map(|s| s.to_string()).collect();

    let s = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let t = first_multiple_input[1].trim().parse::<i32>().unwrap();

    println!("Введіть кординати дерев через пробіл");
    let second_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap().split(' ').map(|s| s.to_string()).collect();

    let a = second_multiple_input[0].trim().parse::<i32>().unwrap();

    let b = second_multiple_input[1].trim().parse::<i32>().unwrap();

    println!("Введіть координати падіння яблук, починаючи з яблуні, через пробіл");
    let apples: Vec<i32> = stdin_iterator.next().unwrap().unwrap().trim_end().split(' ').map(|s| s.to_string().parse::<i32>().unwrap()).collect();

    println!("Введіть координати падіння апельсинів, починаючи з апельсинового дерева, через пробіл");
    let oranges: Vec<i32> = stdin_iterator.next().unwrap().unwrap().trim_end().split(' ').map(|s| s.to_string().parse::<i32>().unwrap()).collect();

    count_apples_and_oranges(s, t, a, b, &apples, &oranges);
}
