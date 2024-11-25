use std::io::{self, BufRead};

/*
 * Complete the 'getTotalX' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let start = *a.iter().max().unwrap();
    let end = *b.iter().min().unwrap();

    let mut count = 0;

    for x in start..=end {
        let mut is_multiple_of_a = true;
        for &ai in a {
            if x % ai != 0 {
                is_multiple_of_a = false;
                break;
            }
        }

        let mut is_factor_of_b = true;
        for &bi in b {
            if bi % x != 0 {
                is_factor_of_b = false;
                break;
            }
        }

        if is_multiple_of_a && is_factor_of_b {
            count += 1;
        }
    }

    count
}

pub fn between_two_sets_main() {
    println!("Task 13. Between Two Sets");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
.trim_end().split(' ').map(|s| s.to_string().parse::<i32>().unwrap()).collect();

    let brr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let total = get_total_x(&arr, &brr);

    println!("{}", total);
}