use std::io::{self, BufRead};

fn migratory_birds(arr: &[i32]) -> i32 {
    let counts = arr.iter().fold([0; 5], |mut counts, &bird| {
        if (1..=5).contains(&bird) {
            counts[(bird - 1) as usize] += 1;
        }
        counts
    });

    counts
        .iter()
        .enumerate()
        .max_by_key(|&(bird_id, &count)| (count, -(bird_id as i32)))
        .map(|(bird_id, _)| bird_id + 1)
        .unwrap() as i32
}

pub fn migratory_birds_main() {
    println!("Task 17. Migratory Birds");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratory_birds(&arr);

    println!("{}", result);
}
