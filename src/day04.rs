use ahash::{HashSet, HashSetExt};
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{bytes::complete::tag, character::complete::space1, IResult};

use crate::common::nom::{fold_separated_list0, nom_lines, nom_u8, process_input};

fn parse_nums(s: &str) -> IResult<&str, HashSet<u8>> {
    fold_separated_list0(space1, nom_u8, HashSet::new, |mut acc, n| {
        acc.insert(n);
        acc
    })(s)
}

fn parse(s: &str) -> IResult<&str, (HashSet<u8>, HashSet<u8>)> {
    let (s, _) = tag("Card")(s)?;
    let (s, _) = space1(s)?;
    let (s, _) = nom_u8(s)?;
    let (s, _) = tag(":")(s)?;
    let (s, _) = space1(s)?;
    let (s, left) = parse_nums(s)?;
    let (s, _) = tag(" |")(s)?;
    let (s, _) = space1(s)?;
    let (s, right) = parse_nums(s)?;

    Ok((s, (left, right)))
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<(HashSet<u8>, HashSet<u8>)> {
    process_input(nom_lines(parse))(input)
}

fn check_part1(winners: &HashSet<u8>, numbers: &HashSet<u8>) -> usize {
    let n = winners.intersection(numbers).count();
    if n == 0 {
        0
    } else {
        2usize.pow(u32::try_from(n - 1).unwrap())
    }
}

#[aoc(day4, part1)]
pub fn part1(inputs: &[(HashSet<u8>, HashSet<u8>)]) -> usize {
    inputs.iter().map(|(a, b)| check_part1(a, b)).sum()
}

#[aoc(day4, part2)]
pub fn part2(inputs: &[(HashSet<u8>, HashSet<u8>)]) -> usize {
    let mut cards = vec![1; inputs.len()];

    for (i, c) in inputs.iter().enumerate() {
        let multipler = cards[i];
        let count = c.0.intersection(&c.1).count();

        for n in 0..count {
            cards[i + 1 + n] += multipler;
        }
    }

    cards.iter().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 13);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 30);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day4.txt");
        const ANSWERS: (usize, usize) = (22193, 5625994);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
