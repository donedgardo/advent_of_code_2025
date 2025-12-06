use advent_of_code_2025::{day02, get_input};

fn main() {
    let answer = part_1_answer("day02.txt");
    println!("day 02 part 1: {}", answer);

    let answer = part_2_answer("day02.txt");
    println!("day 02 part 2: {}", answer);
}
fn part_1_answer(file_name: &str) -> u64 {
    let input = get_input(file_name);
    day02::part_1(input.first().unwrap())
}

fn part_2_answer(file_name: &str) -> u64 {
    let input = get_input(file_name);
    day02::part_2(input.first().unwrap())
}
