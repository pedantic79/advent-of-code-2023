use ahash::{HashMap, HashMapExt};
use aoc_runner_derive::{aoc, aoc_generator};

use crate::common::utils::parse_split;

#[derive(PartialEq, Eq, Clone)]
pub enum State {
    Unknown,
    Damaged,
    Operational,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "?"),
            Self::Damaged => write!(f, "#"),
            Self::Operational => write!(f, "."),
        }
    }
}

fn parse_spring(s: &str) -> Vec<State> {
    s.bytes()
        .map(|b| match b {
            b'.' => State::Operational,
            b'#' => State::Damaged,
            b'?' => State::Unknown,
            _ => panic!("unknown symbol"),
        })
        .collect()
}

#[derive(Debug)]
pub struct Line {
    spring: Vec<State>,
    cond: Vec<usize>,
}

impl Line {
    fn solve(
        &self,
        pos_spring: usize,
        pos_cond: usize,
        stride: usize,
        dp: &mut HashMap<(usize, usize, usize), usize>,
    ) -> usize {
        if let Some(v) = dp.get(&(pos_spring, pos_cond, stride)) {
            return *v;
        }

        if pos_spring == self.spring.len() {
            // if cond end and no stride OR
            // one before the end AND stride matches current cond
            if pos_cond == self.cond.len() && stride == 0
                || pos_cond == self.cond.len() - 1 && stride == self.cond[pos_cond]
            {
                return 1;
            } else {
                return 0;
            }
        }

        let mut res = 0;
        for c in [State::Operational, State::Damaged] {
            if self.spring[pos_spring] == c || self.spring[pos_spring] == State::Unknown {
                let pos = pos_spring + 1;
                res += match c {
                    State::Unknown => unreachable!(),
                    State::Damaged => self.solve(pos, pos_cond, stride + 1, dp),
                    State::Operational if stride == 0 => self.solve(pos, pos_cond, 0, dp),
                    State::Operational
                        // not at the end and we match conditions
                        if pos_cond < self.cond.len() && stride == self.cond[pos_cond] =>
                    {
                        self.solve(pos, pos_cond + 1, 0, dp)
                    }
                    State::Operational => 0,
                };
            }
        }

        dp.insert((pos_spring, pos_cond, stride), res);
        res
    }
}

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let (spring, condition) = line.split_once(' ').unwrap();

            Line {
                spring: parse_spring(spring),
                cond: parse_split(condition, ','),
            }
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn part1(inputs: &[Line]) -> usize {
    let mut dp = HashMap::new();
    inputs
        .iter()
        .map(|l| {
            let n = l.solve(0, 0, 0, &mut dp);
            dp.clear();
            n
        })
        .sum()
}

#[aoc(day12, part2)]
pub fn part2(inputs: &[Line]) -> usize {
    let mut dp = HashMap::new();
    let mut res = 0;

    for l in inputs {
        let mut spring = Vec::with_capacity((l.spring.len() + 1) * 5);
        let mut cond = Vec::with_capacity(l.cond.len() * 5);
        for _ in 0..4 {
            spring.extend_from_slice(&l.spring);
            spring.push(State::Unknown);
            cond.extend_from_slice(&l.cond);
        }
        spring.extend_from_slice(&l.spring);
        cond.extend_from_slice(&l.cond);

        res += Line { spring, cond }.solve(0, 0, 0, &mut dp);
        dp.clear();
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    pub fn input_test() {
        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 21);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 525152);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day12.txt");
        const ANSWERS: (usize, usize) = (7286, 25470469710341);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
