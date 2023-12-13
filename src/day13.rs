use std::convert::Infallible;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::common::utils::parse_split;

#[derive(Debug)]
pub struct Pattern {
    pattern: Vec<Vec<u8>>,
}

impl std::str::FromStr for Pattern {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern: Vec<_> = s.lines().map(|line| line.as_bytes().to_vec()).collect();
        Ok(Pattern { pattern })
    }
}

impl Pattern {
    fn find_mirror<const TARGET: usize>(&self) -> Option<usize> {
        let a = Self::find_mirror_horz::<TARGET>(&self.pattern);

        if let Some(a) = a {
            Some(a * 100)
        } else {
            Self::find_mirror_vert::<TARGET>(&self.pattern)
        }
    }

    fn count_badness(left: &[u8], right: &[u8]) -> usize {
        left.iter()
            .zip(right.iter())
            .filter(|(l, r)| l != r)
            .count()
    }

    fn check_mirror<const TARGET: usize>(left: &[Vec<u8>], right: &[Vec<u8>]) -> bool {
        // left.iter().rev().zip(right.iter()).all(|(l, r)| l == r)

        left.iter()
            .rev()
            .zip(right.iter())
            .map(|(l, r)| Self::count_badness(l, r))
            .sum::<usize>()
            == TARGET
    }

    fn find_mirror_horz<const TARGET: usize>(p: &[Vec<u8>]) -> Option<usize> {
        (1..p.len()).find(|&split| {
            let (left, right) = p.split_at(split);
            Self::check_mirror::<TARGET>(left, right)
        })
    }

    fn find_mirror_vert<const TARGET: usize>(p: &[Vec<u8>]) -> Option<usize> {
        let p = rotate(p);
        Self::find_mirror_horz::<TARGET>(&p)
    }
}

fn rotate(matrix: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut rotated_matrix = vec![vec![0; rows]; cols];

    for i in 0..rows {
        for (j, &cell) in matrix[rows - 1 - i].iter().enumerate() {
            rotated_matrix[j][i] = cell;
        }
    }

    rotated_matrix
}

#[aoc_generator(day13)]
pub fn generator(input: &str) -> Vec<Pattern> {
    parse_split(input, "\n\n")
}

#[aoc(day13, part1)]
pub fn part1(inputs: &[Pattern]) -> usize {
    inputs.iter().filter_map(Pattern::find_mirror::<0>).sum()
}

#[aoc(day13, part2)]
pub fn part2(inputs: &[Pattern]) -> usize {
    inputs.iter().filter_map(Pattern::find_mirror::<1>).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    pub fn input_test() {
        let p = generator(SAMPLE);
        // println!("{:?}", p);

        assert_eq!(Pattern::find_mirror_vert::<0>(&p[0].pattern), Some(5));
        assert_eq!(Pattern::find_mirror_horz::<0>(&p[1].pattern), Some(4));
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 405);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 400);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day13.txt");
        const ANSWERS: (usize, usize) = (33047, 28806);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
