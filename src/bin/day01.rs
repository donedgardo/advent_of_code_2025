use advent_of_code_2025::{day01, get_input};

fn main() {
    let answer = get_day_1_answer("day01.txt");
    println!("answer1: {}", answer);
    let answer2 = get_day_1_answer2("day01.txt");
    println!("answer2: {}", answer2);
}

fn get_day_1_answer(file_name: &str) -> u32 {
    let input = get_input(file_name);
    day01::get_password(input, 50)
}
fn get_day_1_answer2(file_name: &str) -> u32 {
    let input = get_input(file_name);
    day01::get_password2(input, 50)
}

