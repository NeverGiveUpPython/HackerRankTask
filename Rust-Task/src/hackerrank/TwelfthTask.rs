use std::io::{self, BufRead};

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 == v2 {
        return match x1 == x2 {
            true => "Так".to_string(),
            false => "Ні".to_string(),
        };
    }
    
    let (numerator, denominator) = (x2 - x1, v1 - v2);
    
    match (numerator % denominator == 0, numerator / denominator > 0) {
        (true, true) => "Так".to_string(),
        _ => "Ні".to_string(),
    }
}

pub fn number_line_jumps_main() {
    println!("Task 12. Number Line Jumps");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    println!("Введіть точку старту кенгуру та відстань стрибка");
    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap().split(' ').map(|s| s.to_string()).collect();

    let x1 = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let v1 = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let x2 = first_multiple_input[2].trim().parse::<i32>().unwrap();

    let v2 = first_multiple_input[3].trim().parse::<i32>().unwrap();

    let result = kangaroo(x1, v1, x2, v2);

    println!("Кенгуру зустрічається? {}.", result);
}
