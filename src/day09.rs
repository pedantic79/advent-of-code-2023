use aoc_runner_derive::{aoc, aoc_generator};
use nom::{bytes::complete::tag, multi::separated_list0};

use crate::common::nom::{nom_i64, process_input};

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<Vec<Vec<i64>>> {
    let mut res = Vec::new();
    for mut v in input
        .lines()
        .map(|line| process_input(separated_list0(tag(" "), nom_i64::<_, ()>))(line))
    {
        let mut differences = Vec::new();

        while v.iter().any(|&x| x != 0) {
            let v2 = diff(&v);
            differences.push(v);
            v = v2;
        }

        res.push(differences)
    }

    res
}

fn diff(v: &[i64]) -> Vec<i64> {
    v.windows(2).map(|x| x[1] - x[0]).collect()
}

fn process<F>(differences: &[Vec<i64>], op: F) -> i64
where
    F: Fn(&Vec<i64>, i64) -> i64,
{
    differences
        .iter()
        .rev()
        .fold(0, |constant, d| op(d, constant))
}

#[aoc(day9, part1)]
pub fn part1(inputs: &[Vec<Vec<i64>>]) -> i64 {
    inputs
        .iter()
        .map(|x| process(x, |d, c| d.last().copied().unwrap() + c))
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(inputs: &[Vec<Vec<i64>>]) -> i64 {
    inputs
        .iter()
        .map(|x| process(x, |d, c| d.first().copied().unwrap() - c))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 114);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 2);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day9.txt");
        const ANSWERS: (i64, i64) = (1834108701, 993);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
