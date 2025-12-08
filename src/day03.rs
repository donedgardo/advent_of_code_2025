
pub fn part_1(input: Vec<String>) -> u32 {
    input.into_iter().map(|bank| {
        bank.chars().map(|s| s.to_digit(10).unwrap()).collect::<Vec<_>>()
    }).map(|n| get_highest_voltage(n)).sum()
}
pub fn get_highest_voltage(bank: Vec<u32>) -> u32 {
    if bank.len() == 1 {
        return *bank.iter().max().unwrap_or(&0);
    }
    if bank.len() == 2 {
        return bank.iter().fold(0, |acc, &num| {
            acc * 10 + num
        });
    }
    let mut lhm_i: usize = 0;
    let mut max: u32 = 0;
    for (i, n) in bank[0..bank.len() - 1].iter().enumerate() {
        if n > &max {
            max = *n;
            lhm_i = i;
        }
    }
    let left_hand_max = max; // 3
    max = 0;
    for n in bank[lhm_i+1..].iter() {
        if n > &max {
            max = *n;
        }
    }
    let right_hand_max = max;
    left_hand_max * 10 + right_hand_max
}

pub fn part_2(input: Vec<String>) -> u64 {
    input.into_iter().map(|bank| {
        bank.chars().map(|s| s.to_digit(10).unwrap() as u64).collect::<Vec<_>>()
    }).map(|n| get_highest_voltage_12(n)).sum()
}

pub fn get_highest_voltage_12(bank: Vec<u64>) -> u64 {
    if bank.len() <= 12 {
        return bank.iter().enumerate().map(|(i, num)| {
            10u64.pow((bank.len() - 1 - i) as u32) * num
        }).sum::<u64>();
    }
    let mut num_digits = 0;
    let mut lhi: usize = 0;
    let mut acc  = 0;
    while 12 - num_digits > 0  {
        let mut max: u64 = 0;
        let mut cycle = 0;
        for (i, n) in bank[lhi..=(bank.len() - 12 + num_digits)].iter().enumerate() {
            if n > &max {
                max = *n;
                cycle = i;
            }
        }
        lhi = cycle + lhi + 1;
        num_digits += 1;
        acc += 10u64.pow((12 - num_digits) as u32) * max;
    }
    acc
}


#[cfg(test)]
mod day03_test {
    use rstest::rstest;
    use crate::day03::{part_1, part_2, get_highest_voltage, get_highest_voltage_12};
    use crate::get_sample_input;

    #[test]
    fn sample_test_1() {
        let input = get_sample_input("day03.txt");
        let answer = part_1(input);
        assert_eq!(answer, 357);
    }

    #[rstest]
    #[case(vec![1], 1)]
    #[case(vec![1,2], 12)]
    #[case(vec![1,2,3], 23)]
    #[case(vec![3,2,1], 32)]
    #[case(vec![2,3,4,2,7,8], 78)]
    #[case(vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1], 92)]
    fn gets_highest_voltage(#[case] input: Vec<u32>, #[case] expected: u32) {
        let answer = get_highest_voltage(input);
        assert_eq!(answer, expected);
    }

    #[test]
    fn sample_test_2() {
        let input = get_sample_input("day03.txt");
        let answer = part_2(input);
        assert_eq!(answer, 3121910778619);
    }

    #[rstest]
    #[case(vec![1], 1)]
    #[case(vec![1,2], 12)]
    #[case(vec![3,2,1], 321)]
    #[case(vec![2,3,4,2,7,8], 234278)]
    #[case(vec![8,1,8,1,8,1,9,1,1,1,1,2], 818181911112)]
    #[case(vec![2,3,4,2,7,8,8,1,9,2,2,7,8], 342788192278)]
    #[case(vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1], 987654321111)]
    #[case(vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1], 888911112111)]
    fn get_voltage_2(#[case] input: Vec<u64>, #[case] expected: u64) {
        let answer = get_highest_voltage_12(input);
        assert_eq!(answer, expected);
    }
}
