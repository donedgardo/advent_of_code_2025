use advent_of_code_2025::{day05, get_input};

fn main() {
    let answer = part_1_answer("day05.txt");
    println!("day 05 part 1: {}", answer);

    // let answer = part_2_answer("day05.txt");
    // println!("day 05 part 2: {}", answer);
}
fn part_1_answer(file_name: &str) -> u64 {
    let input = get_input(file_name);
    day05::part_1(input)
}

// fn part_2_answer(file_name: &str) -> u64 {
//     let input = get_input(file_name);
//     day05::part_2(input)
// }
