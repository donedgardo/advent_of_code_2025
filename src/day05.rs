pub fn part_1(input: Vec<String>) -> u32 {
    0
}

pub fn part_2(input: Vec<String>) -> u64 {
    0
}



#[cfg(test)]
mod day05_test {
    use crate::day05::{part_1, part_2};
    use crate::get_sample_input;

    #[test]
    #[ignore]
    fn sample_test_1() {
        let input = get_sample_input("day05.txt");
        let answer = part_1(input);
        assert_eq!(answer, 3);
    }

    #[test]
    #[ignore]
    fn sample_test_2() {
        let input = get_sample_input("day05.txt");
        let answer = part_2(input);
        assert_eq!(answer, 43);
    }
}
