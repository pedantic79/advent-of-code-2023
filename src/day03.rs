use ahash::{HashMap, HashMapExt};
use aoc_runner_derive::{aoc, aoc_generator};

use crate::common::utils::neighbors_diag;

#[allow(clippy::type_complexity)]
#[aoc_generator(day3)]
pub fn generator(input: &str) -> (Vec<(u32, bool)>, HashMap<(usize, usize), Vec<u32>>) {
    let v = input.lines().map(|line| line.bytes().collect()).collect();
    let mut hm = HashMap::new();
    let x = parse_numbers(v, &mut hm);
    (x, hm)
}

fn area_check(input: &[Vec<u8>], coords: &[(usize, usize)]) -> (bool, Option<(usize, usize)>) {
    let mut other = false;
    let mut gear = None;
    for &(r, c) in coords.iter() {
        for (y, x) in neighbors_diag(r, c, input.len(), input[0].len()) {
            let cell = input[y][x];
            if cell == b'*' {
                gear = Some((y, x));
            }
            other |= cell != b'.' && !cell.is_ascii_digit();
        }
    }

    (other, gear)
}

fn parse_numbers(
    input: Vec<Vec<u8>>,
    gears: &mut HashMap<(usize, usize), Vec<u32>>,
) -> Vec<(u32, bool)> {
    let mut res = vec![];

    for (row, line) in input.iter().enumerate() {
        let mut num = vec![];
        let mut check = vec![];

        for (col, &b) in line.iter().enumerate() {
            if b.is_ascii_digit() {
                num.push(b);
                check.push((row, col));
            } else if !num.is_empty() {
                let number = unsafe { String::from_utf8_unchecked(num) }.parse().unwrap();
                num = vec![];
                let (surround, gear) = area_check(&input, &check);
                res.push((number, surround));
                check.clear();
                if let Some(co) = gear {
                    gears.entry(co).or_default().push(number);
                }
            }
        }

        if !num.is_empty() {
            let number = unsafe { String::from_utf8_unchecked(num) }.parse().unwrap();
            let (surround, gear) = area_check(&input, &check);
            res.push((number, surround));
            check.clear();
            if let Some(co) = gear {
                gears.entry(co).or_default().push(number);
            }
        }
    }
    res
}

#[allow(clippy::type_complexity)]
#[aoc(day3, part1)]
pub fn part1(inputs: &(Vec<(u32, bool)>, HashMap<(usize, usize), Vec<u32>>)) -> u32 {
    inputs
        .0
        .iter()
        .filter(|x| x.1)
        .map(|x| x.0)
        // .inspect(|x| println!("{x}"))
        .sum()
}

#[allow(clippy::type_complexity)]
#[aoc(day3, part2)]
pub fn part2(inputs: &(Vec<(u32, bool)>, HashMap<(usize, usize), Vec<u32>>)) -> u32 {
    inputs
        .1
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
