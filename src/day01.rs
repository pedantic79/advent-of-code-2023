use aoc_runner_derive::{aoc, aoc_generator};

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

fn find_from_pos(input: &[u8], pos: usize) -> Option<u32> {
    let input = &input[pos..];

    if input[0].is_ascii_digit() {
        return Some(u32::from(input[0] - b'0'));
    }

    let end = 3.min(input.len());

    match &input[0..end] {
        b"one" => Some(1),
        b"two" => Some(2),
        b"thr" => input[end..].starts_with(b"ee").then_some(3),
        b"fou" => input[end..].starts_with(b"r").then_some(4),
        b"fiv" => input[end..].starts_with(b"e").then_some(5),
        b"six" => Some(6),
        b"sev" => input[end..].starts_with(b"en").then_some(7),
        b"eig" => input[end..].starts_with(b"ht").then_some(8),
        b"nin" => input[end..].starts_with(b"e").then_some(9),
        _ => None,
    }
}

fn parse(s: &[u8]) -> Vec<u32> {
    (0..s.len()).filter_map(|i| find_from_pos(s, i)).collect()
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
