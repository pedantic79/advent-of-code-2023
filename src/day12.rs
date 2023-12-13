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
    block: Vec<usize>,
}

impl Line {
    fn solve(&self, dp: &mut HashMap<(usize, usize, usize), usize>) -> usize {
        solve(&self.spring, &self.block, 0, dp)
    }

    fn expand(&self) -> Self {
        let mut spring = Vec::with_capacity((self.spring.len() + 1) * 5);
        let mut block = Vec::with_capacity(self.block.len() * 5);
        for _ in 0..4 {
            spring.extend_from_slice(&self.spring);
            spring.push(State::Unknown);
            block.extend_from_slice(&self.block);
        }
        spring.extend_from_slice(&self.spring);
        block.extend_from_slice(&self.block);

        Self { spring, block }
    }
}

fn solve(
    spring: &[State],
    block: &[usize],
    stride: usize,
    dp: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if let Some(v) = dp.get(&(spring.len(), block.len(), stride)) {
        return *v;
    }

    if spring.is_empty() {
        // if block is empty and stride is empty OR
        // last block is the same as the stride
        if block.is_empty() && stride == 0 || block.len() == 1 && stride == block[0] {
            return 1;
        } else {
            return 0;
        }
    }

    let res = solve_helper(&spring[0], spring, block, stride, dp);
    dp.insert((spring.len(), block.len(), stride), res);
    res
}

fn solve_helper(
    piece: &State,
    spring: &[State],
    block: &[usize],
    stride: usize,
    dp: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    // Damaged                             : increase stride
    // Operational && empty stride         : continue processing
    // Operational && stride == first block: next block, and reset stride
    // Operational && anything else        : Invalid, return 0
    // Unknown                             : Run solve_helper with Damaged + Operational
    match piece {
        State::Damaged => solve(&spring[1..], block, stride + 1, dp),
        State::Operational if stride == 0 => solve(&spring[1..], block, 0, dp),
        State::Operational if block.first() == Some(&stride) => {
            solve(&spring[1..], &block[1..], 0, dp)
        }
        State::Operational => 0,
        State::Unknown => {
            solve_helper(&State::Damaged, spring, block, stride, dp)
                + solve_helper(&State::Operational, spring, block, stride, dp)
        }
    }
}

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let (spring, blockition) = line.split_once(' ').unwrap();

            Line {
                spring: parse_spring(spring),
                block: parse_split(blockition, ','),
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
            // dp.clear();
            let x = l.solve(&mut dp);
            dp.clear();
            x
        })
        .sum()
}

#[aoc(day12, part2)]
pub fn part2(inputs: &[Line]) -> usize {
    let mut dp = HashMap::new();
    inputs
        .iter()
        .map(|l| {
            let x = l.expand().solve(&mut dp);
            dp.clear();
            x
        })
        .sum()
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
