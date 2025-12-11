use advent_of_code_2025::{day06, get_input};

fn main() {
    let answer = part_1_answer("day06.txt");
    println!("day 06 part 1: {}", answer);

    // let answer = part_2_answer("day06.txt");
    // println!("day 06 part 2: {}", answer);
}
fn part_1_answer(file_name: &str) -> u64 {
    let input = get_input(file_name);
    day06::part_1(input)
}

// fn part_2_answer(file_name: &str) -> u64 {
//     let input = get_input(file_name);
//     day05::part_2(input)
// }
