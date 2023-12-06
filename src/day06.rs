use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::{tag, take_until, take_while},
    character::complete::{space0, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};
use num::integer::sqrt;

use crate::common::nom::{fold_separated_list0, nom_lines, nom_usize, process_input};

fn parse_line(s: &str) -> IResult<&str, Vec<usize>> {
    let (s, _) = take_until(":")(s)?;
    let (s, _) = tag(":")(s)?;
    let (s, _) = space0(s)?;

    separated_list1(space1, nom_usize)(s)
}

fn parse_number(s: &str) -> IResult<&str, usize> {
    let (s, _) = take_until(":")(s)?;
    let (s, _) = tag(":")(s)?;
    let (s, _) = space0(s)?;

    map_res(
        fold_separated_list0(
            space1,
            take_while(|x: char| x.is_numeric()),
            String::new,
            |mut acc, x| {
                acc += x;
                acc
            },
        ),
        |x| x.parse(),
    )(s)
}

#[aoc_generator(day6, part1)]
pub fn generator_p1(input: &str) -> Vec<Vec<usize>> {
    process_input(nom_lines(parse_line))(input)
}

#[aoc_generator(day6, part2)]
pub fn generator_p2(input: &str) -> Vec<usize> {
    process_input(nom_lines(parse_number))(input)
}

fn ways(time: usize, dist: usize) -> usize {
    let dist_f = |x| x * time - x * x;

    let pyth = sqrt(time * time - 4 * dist);
    let root1 = (time - pyth) / 2;
    let root2 = (time + pyth) / 2;

    let a = (root1 - 1..root1 + 2).find(|x| dist_f(*x) > dist).unwrap();
    let b = (root2 - 1..root2 + 2).find(|x| dist_f(*x) <= dist).unwrap();
    b - a
}

#[aoc(day6, part1)]
pub fn part1(inputs: &[Vec<usize>]) -> usize {
    inputs[0]
        .iter()
        .zip(&inputs[1])
        .map(|(x, y)| ways(*x, *y))
        .product()
}

#[aoc(day6, part2)]
pub fn part2(inputs: &[usize]) -> usize {
    ways(inputs[0], inputs[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator_p1(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator_p1(SAMPLE)), 288);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator_p2(SAMPLE)), 71503);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day6.txt");
        const ANSWERS: (usize, usize) = (449820, 42250895);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output1 = generator_p1(input);
            let output2 = generator_p2(input);

            assert_eq!(part1(&output1), ANSWERS.0);
            assert_eq!(part2(&output2), ANSWERS.1);
        }
    }
}
