use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::ops::RangeInclusive;

pub fn part_1(input: &String) -> u64 {
    input
        .split(',')
        .collect::<Vec<_>>()
        .par_iter()
        .map(|x| get_invalid_ids(x))
        .sum()
}

pub fn part_2(input: &String) -> u64 {
    input
        .split(',')
        .collect::<Vec<_>>()
        .par_iter()
        .map(|x| get_invalid_ids_2(x))
        .sum()
}

fn get_invalid_ids(input: &str) -> u64 {
    let range = get_range(input);
    range
        .into_par_iter()
        .map(|num| {
            let mut total = 0;
            let num_as_str = num.to_string();
            let length = num_as_str.len();
            if length.rem_euclid(2) != 0 {
                return total;
            }
            let half = length / 2;
            let (first_half, second_half) = num_as_str.split_at(half);
            if first_half == second_half {
                total += num
            }
            total
        })
        .sum::<u64>()
}
pub fn get_invalid_ids_2(input: &str) -> u64 {
    let range = get_range(input);
    range
        .into_par_iter()
        .map(|num| {
            let mut total = 0;
            let num_digits = count_digits(num);
            for chunk_size in 1..=(num_digits / 2) {
                if (num_digits % chunk_size) != 0 {
                    continue;
                }
                if has_repeating_pattern(num, num_digits, chunk_size) {
                    total += num;
                    break;
                }
            }
            total
        })
        .sum::<u64>()
}

fn count_digits(mut n: u64) -> usize {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}

fn has_repeating_pattern(num: u64, num_digits: usize, chunk_size: usize) -> bool {
    let divisor = 10_u64.pow(chunk_size as u32);
    let first_chunk = (num / 10_u64.pow((num_digits - chunk_size) as u32)) % divisor;

    let num_chunks = num_digits / chunk_size;
    for i in 0..num_chunks {
        let shift = (num_digits - (i + 1) * chunk_size) as u32;
        let chunk = (num / 10_u64.pow(shift)) % divisor;
        if chunk != first_chunk {
            return false;
        }
    }
    true
}

fn get_range(input: &str) -> RangeInclusive<u64> {
    let left: u64 = input.split('-').next().unwrap().parse::<u64>().unwrap();
    let end: u64 = input.split('-').nth(1).unwrap().parse::<u64>().unwrap();
    left..=end
}

#[cfg(test)]
mod day02_test {
    use crate::day02::{get_invalid_ids, get_invalid_ids_2, get_range, part_1, part_2};
    use crate::get_sample_input;
    use rstest::rstest;
    use std::ops::RangeInclusive;

    #[test]
    fn sample_test_1() {
        let input = get_sample_input("day02.txt");
        let answer = part_1(input.first().unwrap());
        assert_eq!(answer, 1227775554);
    }

    #[rstest]
    #[case("11-22", 11 + 22)]
    #[case("95-115", 99)]
    #[case("1188511880-1188511890", 1188511885)]
    #[case("222220-222224", 222222)]
    fn finds_invalid_id(#[case] input: &str, #[case] expected: u64) {
        let invalid_ids = get_invalid_ids(input);
        assert_eq!(invalid_ids, expected);
    }

    #[test]
    fn parses_string_range() {
        let input = "11-22";
        let range: RangeInclusive<u64> = get_range(input);
        assert_eq!(range.start(), &11);
        assert_eq!(range.end(), &22);
    }

    #[test]
    fn sample_part_two() {
        let input = get_sample_input("day02.txt");
        let answer = part_2(input.first().unwrap());
        assert_eq!(answer, 4174379265);
    }

    #[rstest]
    #[case("11-22", 11 + 22)]
    #[case("95-115", 99 + 111)]
    #[case("1188511880-1188511890", 1188511885)]
    #[case("222220-222224", 222222)]
    #[case("824824821-824824827", 824824824)]
    fn finds_invalid_id_2(#[case] input: &str, #[case] expected: u64) {
        let invalid_ids = get_invalid_ids_2(input);
        assert_eq!(invalid_ids, expected);
    }
}
