use aoc_runner_derive::{aoc, aoc_generator};

use crate::common::utils::parse_split;

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<Vec<i64>> {
    input.lines().map(|line| parse_split(line, ' ')).collect()
}

fn diff(v: &[i64]) -> Vec<i64> {
    v.windows(2).map(|x| x[1] - x[0]).collect()
}

fn process(v: &[i64]) -> i64 {
    let mut v = v.to_vec();
    let mut differences = vec![v.to_vec()];

    loop {
        let v2 = diff(&v);
        let all_zeros = v2.iter().all(|&x| x == 0);
        if all_zeros {
            break;
        }

        differences.push(v2.clone());

        v = v2;
    }

    let mut constant = 0;
    while let Some(d) = differences.pop() {
        let last = d.last().copied().unwrap() + constant;
        constant = last;
    }

    constant
}

fn process2(v: &[i64]) -> i64 {
    let mut v = v.to_vec();
    let mut differences = vec![v.to_vec()];

    loop {
        let v2 = diff(&v);
        let all_zeros = v2.iter().all(|&x| x == 0);
        if all_zeros {
            break;
        }

        differences.push(v2.clone());

        v = v2;
    }

    let mut constant = 0;
    while let Some(d) = differences.pop() {
        let last = d.first().copied().unwrap() - constant;
        constant = last;
    }

    constant
}

#[aoc(day9, part1)]
pub fn part1(inputs: &[Vec<i64>]) -> i64 {
    inputs.iter().map(|x| process(x)).sum()
}

#[aoc(day9, part2)]
pub fn part2(inputs: &[Vec<i64>]) -> i64 {
    inputs.iter().map(|x| process2(x)).sum()
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
