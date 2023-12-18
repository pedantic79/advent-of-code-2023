use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{hex_digit1, one_of, space1},
    combinator::{map, map_res},
    IResult,
};

use crate::common::nom::{nom_lines, nom_usize, process_input};

#[derive(Debug, PartialEq, Eq)]
pub enum Dir {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DigDir {
    dir: Dir,
    amt: usize,
}

impl DigDir {
    fn new(dir: Dir, amt: usize) -> Self {
        DigDir { dir, amt }
    }
}

fn parse_line(s: &str) -> IResult<&str, (DigDir, DigDir)> {
    let (s, d) = map(one_of("ULDR"), |x| match x {
        'U' => Dir::Up,
        'L' => Dir::Left,
        'D' => Dir::Down,
        'R' => Dir::Right,
        _ => unreachable!(),
    })(s)?;
    let (s, _) = space1(s)?;
    let (s, n) = nom_usize(s)?;
    let (s, _) = space1(s)?;
    let (s, _) = tag("(#")(s)?;
    let (s, (len, dir)) = map_res(hex_digit1, |x: &str| {
        assert_eq!(x.len(), 6);
        let len = usize::from_str_radix(&x[..5], 16)?;
        let dir = match &x[5..] {
            "3" => Dir::Up,
            "2" => Dir::Left,
            "1" => Dir::Down,
            "0" => Dir::Right,
            _ => panic!("wtf is {:?}", &x[5..]),
        };
        Ok::<_, std::num::ParseIntError>((len, dir))
    })(s)?;
    let (s, _) = tag(")")(s)?;

    Ok((s, (DigDir::new(d, n), DigDir::new(dir, len))))
}

#[aoc_generator(day18)]
pub fn generator(input: &str) -> Vec<(DigDir, DigDir)> {
    process_input(nom_lines(parse_line))(input)
}

fn solve<'a>(itr: impl IntoIterator<Item = &'a DigDir>) -> usize {
    let mut width = 0;
    let mut area = 1;

    for digdir in itr {
        let DigDir { dir: d, amt: l } = digdir;

        match d {
            Dir::Up => {
                area -= width * l;
            }
            Dir::Left => {
                width -= l;
            }
            Dir::Down => {
                area += (width + 1) * l;
            }
            Dir::Right => {
                width += l;
                area += l;
            }
        }
    }

    area
}

#[aoc(day18, part1)]
pub fn part1(inputs: &[(DigDir, DigDir)]) -> usize {
    solve(inputs.iter().map(|(p1, _)| p1))
}

#[aoc(day18, part2)]
pub fn part2(inputs: &[(DigDir, DigDir)]) -> usize {
    solve(inputs.iter().map(|(_, p2)| p2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 62);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 952408144115);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day18.txt");
        const ANSWERS: (usize, usize) = (48503, 148442153147147);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
