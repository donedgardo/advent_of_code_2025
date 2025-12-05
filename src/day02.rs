use std::ops::{RangeInclusive};

pub fn day_2_part_1(input: &String) -> u64 {
    0
}

fn get_invalid_ids(input: &str) -> Vec<u64> {
    let range = get_range(input);
    vec![*range.start(), *range.end()]
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
    use crate::day02::{day_2_part_1, get_invalid_ids, get_range};
    use crate::get_sample_input;

    #[test]
    #[ignore]
    fn sample_test_1() {
        let input = get_sample_input("day02.txt");
        let answer = day_2_part_1(input.first().unwrap());
        assert_eq!(answer, 1227775554);
    }

    #[rstest]
    #[case("11-22", vec![11, 22])]
    #[case("95-115", vec![99])]
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


}
