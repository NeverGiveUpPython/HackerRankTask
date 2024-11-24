use std::io::{self, BufRead};

fn a_very_big_sum(ar: &[i64]) -> i64 { // Обчислює суму чисел масиву
    ar.iter().sum() // .iter - для ітерації по масиву, .sum() - обчислює  суму чисел масиву

pub fn A_Very_Big_Sum_main() {
    println!("Task3. A-Very-Big-Sum");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    println!("Введіть кількість елементів масиву:");
    let _ar_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    println!("Введіть всі числа масиву через пробіл: ");
    let ar: Vec<i64> = stdin_iterator.next().unwrap().unwrap().trim_end().split(' ').map(|s| s.to_string().parse::<i64>().unwrap()).collect();

    if ar.len() != _ar_count as usize { // Обробка помилки коли розмір масиву не співпадає з кількістю елементів. eprintln! -
        eprintln!( //eprintln! - використовується для виводу повідомлення про помилку в консоль
            "Увага: Очікувалось {} елент(-ів) в масиві, але отримав {}.",_ar_count,ar.len()
        );
    }

    let result = a_very_big_sum(&ar); //Сума чисел масиву

    println!("{}", result);
}