use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::{tag, take_until, take_while},
    character::complete::{space0, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

use crate::common::nom::{fold_separated_list0, nom_lines, nom_usize, process_input};

#[derive(Debug, PartialEq, Eq)]
pub struct Object {}

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

#[aoc_generator(day6)]
pub fn generator(input: &str) -> (Vec<Vec<usize>>, Vec<usize>) {
    (
        process_input(nom_lines(parse_line))(input),
        process_input(nom_lines(parse_number))(input),
    )
}

fn ways(time: usize, distance: usize) -> usize {
    (0..time)
        .filter(|speed| speed * (time - speed) > distance)
        .count()
}

#[aoc(day6, part1)]
pub fn part1(inputs: &(Vec<Vec<usize>>, Vec<usize>)) -> usize {
    inputs.0[0]
        .iter()
        .zip(&inputs.0[1])
        .map(|(x, y)| ways(*x, *y))
        .product()
}

#[aoc(day6, part2)]
pub fn part2(inputs: &(Vec<Vec<usize>>, Vec<usize>)) -> usize {
    ways(inputs.1[0], inputs.1[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 288);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 71503);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day6.txt");
        const ANSWERS: (usize, usize) = (449820, 42250895);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
