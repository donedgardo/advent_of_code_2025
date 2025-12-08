use advent_of_code_2025::{day03, get_input};

fn main() {
    let answer = part_1_answer("day03.txt");
    println!("day 03 part 1: {}", answer);

    let answer = part_2_answer("day03.txt");
    println!("day 03 part 2: {}", answer);
}
fn part_1_answer(file_name: &str) -> u32 {
    let input = get_input(file_name);
    day03::part_1(input)
}

fn part_2_answer(file_name: &str) -> u64 {
    let input = get_input(file_name);
    day03::part_2(input)
}
