use std::cmp::Reverse;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    character::complete::{one_of, space1},
    combinator::map,
    sequence::tuple,
    IResult,
};

use crate::common::nom::{nom_lines, nom_usize, process_input};

type HandValue = Reverse<u8>;

fn parse_card(s: &str) -> IResult<&str, HandValue> {
    map(one_of("23456789TJQKA"), |c: char| {
        Reverse(match c {
            '2'..='9' => c as u8 - b'0',
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("unknown symbol"),
        })
    })(s)
}

fn parse_hand(s: &str) -> IResult<&str, Vec<Vec<HandValue>>> {
    map(
        tuple((parse_card, parse_card, parse_card, parse_card, parse_card)),
        |x| {
            let mut cards = [x.0, x.1, x.2, x.3, x.4];
            cards.sort_by_key(|&x| x);
            let mut data_grouped: Vec<Vec<HandValue>> = Vec::new();

            for (_, group) in &cards.into_iter().group_by(|elt| *elt) {
                data_grouped.push(group.collect());
            }

            data_grouped.sort_by_key(|x| Reverse(x.len()));

            data_grouped
        },
    )(s)
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum HandTypes {
    Five(HandValue),
    Four(HandValue, HandValue),
    Full(HandValue, HandValue),
    Three(HandValue, HandValue, HandValue),
    TwoPair(HandValue, HandValue, HandValue),
    OnePair(HandValue, HandValue, HandValue, HandValue),
    High([HandValue; 5]),
}

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Vec<HandValue>>,
    value: usize,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.classify().cmp(&self.classify())
    }
}

impl Hand {
    fn classify(&self) -> HandTypes {
        match self.cards[0].len() {
            5 => HandTypes::Five(self.cards[0][0]),
            4 => HandTypes::Four(self.cards[0][0], self.cards[1][0]),
            3 if self.cards[1].len() == 2 => HandTypes::Full(self.cards[0][0], self.cards[1][0]),
            3 => HandTypes::Three(self.cards[0][0], self.cards[1][0], self.cards[2][0]),
            2 if self.cards[1].len() == 2 => {
                HandTypes::TwoPair(self.cards[0][0], self.cards[1][0], self.cards[2][0])
            }
            2 => HandTypes::OnePair(
                self.cards[0][0],
                self.cards[1][0],
                self.cards[2][0],
                self.cards[3][0],
            ),
            1 => HandTypes::High([
                self.cards[0][0],
                self.cards[1][0],
                self.cards[2][0],
                self.cards[3][0],
                self.cards[4][0],
            ]),
            _ => panic!("unknown hand type: {:?}", self.cards),
        }
    }
}

fn parse_line(s: &str) -> IResult<&str, Hand> {
    let (s, cards) = parse_hand(s)?;
    let (s, _) = space1(s)?;
    let (s, value) = nom_usize(s)?;
    Ok((s, Hand { cards, value }))
}

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<Hand> {
    let mut v = process_input(nom_lines(parse_line))(input);
    v.sort();
    v
}

#[aoc(day7, part1)]
pub fn part1(inputs: &[Hand]) -> usize {
    inputs
        .iter()
        .enumerate()
        .inspect(|x| println!("{:?} {:?}", x.1.classify(), x.1))
        .inspect(|x| println!("{:?}", x.1.value))
        .map(|(place, hand)| hand.value * (place + 1))
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(inputs: &[Hand]) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 6440);
    }

    #[test]
    pub fn part2_test() {
        // assert_eq!(part2(&generator(SAMPLE)), 336);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day7.txt");
        const ANSWERS: (usize, usize) = (0, 0);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            // let output = generator(input);

            // assert_eq!(part1(&output), ANSWERS.0);
            // assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
