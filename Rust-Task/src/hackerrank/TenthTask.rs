use std::io::{self, BufRead};

fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades.into_iter().map(|grade| {
        if grade >= &40 {
            let next_multiple_of_5 = ((grade + 4) / 5) * 5; // Обчислення наступного кратного 5
            if next_multiple_of_5 - grade < 3 {
                return next_multiple_of_5; // Повертаємо округлену оцінку
            }
        }
    *grade 
    }).collect()
}

pub fn grading_students_main() {
    println!("Task 10. Grading Students");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    println!("Введіть кількість оцінок: ");
    let grades_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut grades: Vec<i32> = Vec::with_capacity(grades_count as usize);

    println!("Введіть оцінки через пробіл: ");
    for _ in 0..grades_count {
        let grades_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        grades.push(grades_item);
    }

    let result = grading_students(&grades);

    println!("Заокруглені оцінки {:?}", result);
}