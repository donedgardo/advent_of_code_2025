use std::fmt::Debug;
use std::str::FromStr;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn part_1(input: Vec<String>) -> u64 {
    let (ranges, ingredients) = parse_input::<u64>(input);
    ingredients.into_par_iter().map(|i| {
        for range in ranges.iter() {
            if is_in_range(range.clone(), i) {
                return 1;
            }
        }
        0
    }).sum::<u64>()
}

fn parse_input<T: PartialOrd + FromStr>(input: Vec<String>) -> (Vec<(T, T)>, Vec<T>) where <T as FromStr>::Err: Debug {
    let mut ranges = vec![];
    let mut ingredients = vec![];
    input.into_iter().for_each(|s| {
        if s.contains('-') {
          let range_parts: Vec<&str> = s.split('-').collect();
          let start: T = range_parts[0].parse().unwrap();
          let end: T = range_parts[1].parse().unwrap();
          ranges.push((start, end));
        }  else if !s.is_empty() {
            ingredients.push(s.parse::<T>().unwrap())
        }
    });
    (ranges, ingredients)
}

fn is_in_range<T: PartialOrd>((start, end): (T, T), ingredient: T) -> bool {
    ingredient >= start && ingredient <= end
}

pub fn part_2(_input: Vec<String>) -> u64 {
    0
}



#[cfg(test)]
mod day05_test {
    use rstest::rstest;
    use crate::day05::{is_in_range, parse_input, part_1, part_2};
    use crate::get_sample_input;

    #[test]
    fn sample_test_1() {
        let input = get_sample_input("day05.txt");
        let answer = part_1(input);
        assert_eq!(answer, 3);
    }

    #[rstest]
    #[case(vec![(3, 5), (10,14), (16,20), (12,18)], vec![1, 5, 8, 11, 17, 32])]
    fn parses_input(#[case] expected_ranges: Vec<(u32, u32)>, #[case] expected_ingredients: Vec<u32>) {
        let input = get_sample_input("day05.txt");
        let (ranges, ingredients) = parse_input::<u32>(input);
        assert_eq!(ranges, expected_ranges);
        assert_eq!(ingredients, expected_ingredients);
    }

    #[rstest]
    #[case((0, 1), 1, true)]
    #[case((0, 1), 2, false)]
    fn checks_in_range(
        #[case] range: (u32, u32),
        #[case] ingredient: u32,
        #[case] expected: bool
    ) {
        let answer = is_in_range::<u32>(range, ingredient);
        assert_eq!(answer, expected);
    }

    #[test]
    #[ignore]
    fn sample_test_2() {
        let input = get_sample_input("day05.txt");
        let answer = part_2(input);
        assert_eq!(answer, 43);
    }
}
