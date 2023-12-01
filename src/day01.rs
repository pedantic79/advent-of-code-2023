use aoc_runner_derive::{aoc, aoc_generator};
use nom::{branch::alt, bytes::complete::tag, combinator::value, IResult};

#[aoc_generator(day1, part1)]
pub fn generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .filter(|x| x.is_ascii_digit())
                .map(|b| (b - b'0') as u32)
                .collect()
        })
        .collect()
}

fn parse_digit_ascii(s: &[u8]) -> IResult<&[u8], u32> {
    alt((
        value(8, tag("eight")),
        value(5, tag("five")),
        value(4, tag("four")),
        value(9, tag("nine")),
        value(1, tag("one")),
        value(7, tag("seven")),
        value(6, tag("six")),
        value(3, tag("three")),
        value(2, tag("two")),
        value(0, tag("zero")),
    ))(s)
}

fn parse(s: &[u8]) -> Vec<u32> {
    (0..s.len())
        .filter_map(|i| {
            if s[i].is_ascii_digit() {
                Some(u32::from(s[i] - b'0'))
            } else if let Ok((_, n)) = parse_digit_ascii(&s[i..]) {
                Some(n)
            } else {
                None
            }
        })
        .collect()
}

#[aoc_generator(day1, part2)]
pub fn generator_two(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(|l| parse(l.as_bytes())).collect()
}

#[aoc(day1, part1)]
pub fn part1(inputs: &[Vec<u32>]) -> u32 {
    inputs
        .iter()
        .map(|l: &Vec<u32>| l.first().unwrap() * 10 + l.last().unwrap())
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(inputs: &[Vec<u32>]) -> u32 {
    inputs
        .iter()
        .map(|l: &Vec<u32>| l.first().unwrap() * 10 + l.last().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const SAMPLE2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator_two("ddgjgcrssevensix37twooneightgt"));

        // assert_eq!(generator_two(SAMPLE2), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 142);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator_two(SAMPLE2)), 281);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day1.txt");
        const ANSWERS: (u32, u32) = (53651, 53894);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            // let output = generator(input);

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator_two(input)), ANSWERS.1);
        }
    }
}
