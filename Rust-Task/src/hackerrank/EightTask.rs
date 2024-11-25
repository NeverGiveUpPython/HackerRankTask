use std::io::{self, BufRead};

fn birthday_cake_candles(candles: &[i32]) -> i32 {
    let highest_candle = *candles.iter().max().unwrap();
    candles.iter().filter(|&&candle| candle == highest_candle).count() as i32
}

pub fn birthday_cake_candles_main() {
    println!("Task 8. Birthday Cake Candles");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    println!("Введіть кількість свічок:");
    let _candles_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    println!("Введіть висоту всіх свічок через пробіл: ");
    let candles: Vec<i32> = stdin_iterator.next().unwrap().unwrap().trim_end().split(' ').map(|s| s.to_string().parse::<i32>().unwrap()).collect();

    let result = birthday_cake_candles(&candles);

    println!("Кількість найвищих свічок: {}", result);
}