use ahash::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use arrayvec::ArrayVec;

use crate::common::utils::neighbors_diag;

#[derive(Debug, Default)]
pub struct Map {
    nums: HashMap<(usize, usize), ArrayVec<u32, 2>>,
    none: Vec<u32>,
}

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Map {
    parse_numbers(input.lines().map(|line| line.bytes().collect()).collect())
}

fn parse_numbers(input: Vec<Vec<u8>>) -> Map {
    let mut gears = Map::default();

    for (row, line) in input.iter().enumerate() {
        let mut num = ArrayVec::new();
        let mut check = ArrayVec::new();

        for (col, &b) in line.iter().enumerate() {
            if b.is_ascii_digit() {
                num.push(b);
                check.push((row, col));
            } else if !num.is_empty() {
                process(&input, &mut num, &mut check, &mut gears);
            }
        }

        if !num.is_empty() {
            process(&input, &mut num, &mut check, &mut gears);
        }
    }
    gears
}

fn process(
    input: &[Vec<u8>],
    num: &mut ArrayVec<u8, 3>,
    check: &mut ArrayVec<(usize, usize), 3>,
    gears: &mut Map,
) {
    let number: u32 = parse_int(num);
    let (surround, gear) = area_check(input, check);
    if let Some(co) = gear {
        gears.nums.entry(co).or_default().push(number);
    } else if surround {
        gears.none.push(number);
    }

    num.clear();
    check.clear();
}

fn parse_int(num: &[u8]) -> u32 {
    num.iter()
        .fold(0, |acc, digit| acc * 10 + u32::from(*digit - b'0'))
}

fn area_check(input: &[Vec<u8>], coords: &[(usize, usize)]) -> (bool, Option<(usize, usize)>) {
    for &(r, c) in coords.iter() {
        for (y, x) in neighbors_diag(r, c, input.len(), input[0].len()) {
            let cell = input[y][x];
            if cell != b'.' && !cell.is_ascii_digit() {
                return (true, Some((y, x)).filter(|_| cell == b'*'));
            }
        }
    }

    (false, None)
}

#[aoc(day3, part1)]
pub fn part1(inputs: &Map) -> u32 {
    inputs.nums.values().flatten().sum::<u32>() + inputs.none.iter().sum::<u32>()
}

#[aoc(day3, part2)]
pub fn part2(inputs: &Map) -> u32 {
    inputs
        .nums
        .values()
        .filter(|x| x.len() > 1)
        .map(|x| x.iter().product::<u32>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    pub fn input_test() {
        println!("{:?}", &generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 4361);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 467835);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day3.txt");
        const ANSWERS: (u32, u32) = (528819, 80403602);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
