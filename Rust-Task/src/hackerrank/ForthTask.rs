use std::io::{self, BufRead};

fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();

    let (primary_diagonal_sum, secondary_diagonal_sum) = arr.iter().enumerate().fold((0, 0), |(primary, secondary), (i, row)| {
        (primary + row[i], secondary + row[n - i - 1])
    });

    (primary_diagonal_sum - secondary_diagonal_sum).abs()
}

pub fn diagonal_difference_main() {
    println!("Task 4. Diagonal-Difference");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    println!("Введіть розмір матриці"); // Введіть розмір матриці
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    println!("Введіть значення квадратної матриці:"); // Введіть значення квадратної матриці:
    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        arr.push(Vec::with_capacity(n as usize));


        arr[i] = stdin_iterator.next().unwrap().unwrap().trim_end().split(' ').map(|s| s.to_string().parse::<i32>().unwrap()).collect();
    }
    
    let result = diagonal_difference(&arr);

    println!("Результат: {}", result); // Виводить результат виконання
} 