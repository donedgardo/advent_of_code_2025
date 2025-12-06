use std::ops::{RangeInclusive};

pub fn part_1(input: &String) -> u64 {
    input.split(',')
        .map(|x| get_invalid_ids(x))
        .flatten()
        .sum()
}

pub fn part_2(input: &String) -> u64 {
    input.split(',')
        .map(|x| get_invalid_ids_2(x))
        .flatten()
        .sum()
}

fn get_invalid_ids(input: &str) -> Vec<u64> {
    let mut ids = vec![];
    let range = get_range(input);
    for num in range {
        let num_as_str = num.to_string();
        let length = num_as_str.len();
        if length.rem_euclid(2) != 0 { continue }
        let half =  length / 2;
        let (first_half, second_half) = num_as_str.split_at(half);
        if first_half == second_half { ids.push(num) }
    }
    ids
}
pub fn get_invalid_ids_2(input: &str) -> Vec<u64> {
    let mut ids = vec![];
    let range = get_range(input);
    for num in range {
        let num_as_str = num.to_string();
        let length = num_as_str.len();
        for n in 1..length {
            if (length-n).rem_euclid(n) != 0 { continue }
            let chunks: Vec<String> = num_as_str
                .as_bytes()
                .chunks(n)
                .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap())
                .collect();
            if chunks.iter().all(|item| item == chunks.first().unwrap()) {
                ids.push(num);
                break;
            }
        }
    }
    ids
}

fn get_range(input: &str) -> RangeInclusive<u64> {
    let left: u64 = input.split('-').next().unwrap().parse::<u64>().unwrap();
    let end: u64 = input.split('-').nth(1).unwrap().parse::<u64>().unwrap();
    left..=end
}

#[cfg(test)]
mod day02_test {
    use rstest::rstest;
    use std::ops::{RangeInclusive};
    use crate::day02::{part_1, get_invalid_ids, get_invalid_ids_2, get_range, part_2};
    use crate::get_sample_input;

    #[test]
    fn sample_test_1() {
        let input = get_sample_input("day02.txt");
        let answer = part_1(input.first().unwrap());
        assert_eq!(answer, 1227775554);
    }

    #[rstest]
    #[case("11-22", vec![11, 22])]
    #[case("95-115", vec![99])]
    #[case("1188511880-1188511890", vec![1188511885])]
    #[case("222220-222224", vec![222222])]
    fn finds_invalid_id(#[case] input: &str, #[case] expected: Vec<u64>) {
        let invalid_ids = get_invalid_ids(input);
        assert_eq!(invalid_ids, expected);
    }

    #[test]
    fn parses_string_range() {
        let input = "11-22";
        let range: RangeInclusive<u64>  = get_range(input);
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
    #[case("11-22", vec![11, 22])]
    #[case("95-115", vec![99, 111])]
    #[case("1188511880-1188511890", vec![1188511885])]
    #[case("222220-222224", vec![222222])]
    #[case("824824821-824824827", vec![824824824])]
    fn finds_invalid_id_2(#[case] input: &str, #[case] expected: Vec<u64>) {
        let invalid_ids = get_invalid_ids_2(input);
        assert_eq!(invalid_ids, expected);
    }
}
