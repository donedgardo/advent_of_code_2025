

pub fn part_1(input: Vec<String>) -> u64 {
    let str_input = input.iter().map(|s| s.as_str()).collect();
    let (problem_iter, operations) = parse_input(str_input);
    problem_iter
        .enumerate()
        .map(|(column_index, problem)| {
            problem.iter().fold(0, |acc, n| {
                match operations[column_index] {
                    Operations::Add => acc + n,
                    Operations::Multiply => {
                        if acc == 0 {
                            return 1 * n;
                        }
                        acc * n
                    }
                }
            })
        })
        .sum::<u64>()
}

#[derive(Debug, PartialEq)]
enum Operations {
    Add,
    Multiply,
}

fn parse_input(input: Vec<&str>) -> (impl Iterator<Item = Vec<u64>>, Vec<Operations>) {
    let problems = input[0..input.len() - 1]
        .iter()
        .map(|r| {
            r.split(' ')
                .filter(|n| !n.trim().is_empty())
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();
    let problem_count = problems.first().unwrap_or(&vec![]).len();
    let operations = parse_operations(input.last().unwrap_or(&""));
    let iter = (0..problem_count).into_iter().map(move |column_index| {
        (0..problems.len())
            .into_iter()
            .map(|row_index| problems[row_index][column_index])
            .collect::<Vec<u64>>()
    });
    (iter, operations)
}

fn parse_operations(input: &str) -> Vec<Operations> {
    input
        .chars()
        .filter(|s| !s.is_whitespace())
        .map(|c| match c {
            '+' => Operations::Add,
            '*' => Operations::Multiply,
            _ => panic!("Invalid operation character"),
        })
        .collect()
}

pub fn part_2(_input: Vec<String>) -> u64 {
    todo!()
}

#[cfg(test)]
mod day06_test {
    use crate::day06::{Operations, parse_input, parse_operations, part_1, part_2};
    use crate::get_sample_input;
    use rstest::rstest;

    #[test]
    fn sample_test_1() {
        let input = get_sample_input("day06.txt");
        let answer = part_1(input);
        assert_eq!(answer, 4277556);
    }

    #[rstest]
    #[case(
        vec!["1  2 3 4","5 34  2 43", "+ * * +"],
        (vec![vec!(1, 5), vec!(2, 34), vec!(3, 2), vec!(4, 43)],
        vec![Operations::Add, Operations::Multiply, Operations::Multiply, Operations::Add])
    )]
    fn parses_input(
        #[case] input: Vec<&str>,
        #[case] (expected_problems, expected_operations): (Vec<Vec<u64>>, Vec<Operations>),
    ) {
        let (problems_iter, operations) = parse_input(input);
        assert_eq!(problems_iter.collect::<Vec<Vec<u64>>>(), expected_problems);
        assert_eq!(operations, expected_operations);
    }

    #[rstest]
    #[case("", vec![])]
    #[case("*", vec![Operations::Multiply])]
    #[case("+", vec![Operations::Add])]
    #[case("+ *", vec![Operations::Add, Operations::Multiply])]
    #[case("*   +   *", vec![Operations::Multiply, Operations::Add, Operations::Multiply])]
    fn it_parses_operations(#[case] input: &str, #[case] expected: Vec<Operations>) {
        let operations = parse_operations(input);
        assert_eq!(operations, expected)
    }

    #[test]
    #[ignore]
    fn sample_test_2() {
        let input = get_sample_input("day05.txt");
        let answer = part_2(input);
        assert_eq!(answer, 14);
    }
}
