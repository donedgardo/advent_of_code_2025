use advent_of_code_2025::{day02, day03, day04, get_input};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn day2_part1() {
    day02::part_1(divan::black_box(get_input("day02.txt").first().unwrap()));
}

#[divan::bench]
fn day2_part2() {
    day02::part_2(divan::black_box(get_input("day02.txt").first().unwrap()));
}

#[divan::bench]
fn day3_part1() {
    day03::part_1(divan::black_box(get_input("day03.txt")));
}

#[divan::bench]
fn day3_part2() {
    day03::part_2(divan::black_box(get_input("day03.txt")));
}

#[divan::bench]
fn day4_part1() {
    day04::part_1(divan::black_box(get_input("day04.txt")));
}

#[divan::bench]
fn day4_part2() {
    day04::part_2(divan::black_box(get_input("day04.txt")));
}
