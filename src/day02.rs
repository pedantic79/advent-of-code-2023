use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    multi::separated_list1,
    sequence::{pair, preceded, tuple},
    IResult,
};

use crate::common::nom::{fold_separated_list0, nom_lines, nom_usize, process_input};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeSet {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn product(&self) -> usize {
        self.red * self.green * self.blue
    }
}

fn parse_color(s: &str) -> IResult<&str, (usize, &str)> {
    pair(nom_usize, alt((tag(" red"), tag(" green"), tag(" blue"))))(s)
}

fn parse_cube_set(s: &str) -> IResult<&str, CubeSet> {
    let (s, v) = separated_list1(tag(", "), parse_color)(s)?;
    assert!(v.len() <= 3);
    let (mut red, mut green, mut blue) = (0, 0, 0);
    for (count, color) in v {
        match color {
            " red" => red = count,
            " green" => green = count,
            " blue" => blue = count,
            _ => panic!("unknown color"),
        };
    }

    Ok((s, CubeSet { red, green, blue }))
}

fn parse_merged_cube_set(s: &str) -> IResult<&str, CubeSet> {
    // separated_list1(tag("; "), parse_cube_set)
    fold_separated_list0(tag("; "), parse_cube_set, CubeSet::default, |acc, c| {
        let red = acc.red.max(c.red);
        let green = acc.green.max(c.green);
        let blue = acc.blue.max(c.blue);
        CubeSet { red, green, blue }
    })(s)
}

fn parse_line(s: &str) -> IResult<&str, CubeSet> {
    preceded(
        tuple((tag("Game "), nom_usize, tag(": "))),
        parse_merged_cube_set,
    )(s)
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<CubeSet> {
    process_input(nom_lines(parse_line))(input)
}

#[aoc(day2, part1)]
pub fn part1(inputs: &[CubeSet]) -> usize {
    inputs
        .iter()
        .enumerate()
        .map(|(i, cubes)| if cubes.is_valid() { i + 1 } else { 0 })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(inputs: &[CubeSet]) -> usize {
    inputs.iter().map(|sets| sets.product()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 8);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 2286);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day2.txt");
        const ANSWERS: (usize, usize) = (2771, 70924);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
