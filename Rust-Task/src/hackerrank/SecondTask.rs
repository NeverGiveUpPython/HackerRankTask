use std::io::{self, BufRead};

fn appraisal_triplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut appraisal_result: Vec<i32> = vec![0, 0]; // [Alice's score, Bob's score]

    for i in 0..3 { // Assuming a and b each have 3 elements
        if a[i] > b[i] {
            appraisal_result[0] += 1; // Alice gets a point
        } else if a[i] < b[i] {
            appraisal_result[1] += 1; // + бал Бобу
        }
    }

    appraisal_result
}

pub fn Compare_The_Triplets_main() {
    println!("Task 2. Compare the Triplets");
    let stdin = io::stdin();
    let mut stdin_iterator: io::Lines<io::StdinLock<'_>> = stdin.lock().lines();
    
    println!("Введіть бали Аліси: ");   
    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap().trim_end().split(' ').map(|s: &str| s.to_string().parse::<i32>().unwrap()).collect();

    
    println!("Введіть бали Боба: ");
    let b: Vec<i32> = stdin_iterator.next().unwrap().unwrap().trim_end().split(' ').map(|s: &str| s.to_string().parse::<i32>().unwrap()).collect();

    let result: Vec<i32> = appraisal_triplets(&a, &b);

    println!("Результат порівняння: {:?}", result);
}