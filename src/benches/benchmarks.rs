use advent_of_code_2025::{day02, get_input};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    day02::part_1(divan::black_box(get_input("day02.txt").first().unwrap()));
}

#[divan::bench]
fn part2() {
    day02::part_2(divan::black_box(get_input("day02.txt").first().unwrap()));
}