use std::io::{self, BufRead};

fn breaking_records(scores: &[i32]) -> Vec<i32> {
    if scores.is_empty() {
        return vec![0, 0];
    }

    let mut highest = scores[0];
    let mut lowest = scores[0];
    let counts = scores.iter().skip(1).fold((0, 0), |(high_break_count, low_break_count), &score| {
        if score > highest {
            highest = score;
            (high_break_count + 1, low_break_count)
        } else if score < lowest {
            lowest = score;
            (high_break_count, low_break_count + 1)
        } else {
            (high_break_count, low_break_count)
        }
    });

    vec![counts.0, counts.1]
}

pub fn breaking_best_and_worst_records_main() {
    println!("Task 14. Breaking the Records");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap().trim_end().split(' ').map(|s| s.to_string().parse::<i32>().unwrap()).collect();

    let result = breaking_records(&scores);

    println!("{:?}", result);
}