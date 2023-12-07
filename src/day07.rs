use std::cmp::{Ordering, Reverse};

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

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum HandTypes {
    Five,
    Four,
    Full,
    Three,
    TwoPair,
    OnePair,
    High,
}

#[derive(Debug)]
pub struct Hand {
    cards: Vec<HandValue>,
    value: usize,
}

impl Hand {
    fn classify(&self, joker: u8) -> HandTypes {
        let mut cards: Vec<HandValue> = self
            .cards
            .iter()
            .filter(|x| x.0 != joker)
            .copied()
            .collect();
        let jokers = self.cards.iter().filter(|x| x.0 == joker).count();

        cards.sort_by_key(|&x| x);
        let mut data_grouped: Vec<Vec<HandValue>> = Vec::new();

        for (_, group) in &cards.into_iter().group_by(|elt| *elt) {
            data_grouped.push(group.collect());
        }

        data_grouped.sort_by_key(|x| Reverse(x.len()));
        let cards = data_grouped;

        if cards.is_empty() {
            return HandTypes::Five;
        }
        match cards[0].len() + jokers {
            5 => HandTypes::Five,
            4 => HandTypes::Four,
            3 if cards[1].len() == 2 => HandTypes::Full,
            3 => HandTypes::Three,
            2 if cards[1].len() == 2 => HandTypes::TwoPair,
            2 => HandTypes::OnePair,
            1 => HandTypes::High,
            _ => panic!("unknown hand type: {:?}", self.cards),
        }
    }

    fn compare(&self, other: &Self, joker: u8) -> Ordering {
        other
            .classify(joker)
            .cmp(&self.classify(joker))
            .then_with(|| other.cards.cmp(&self.cards))
    }
}

fn parse_card(part2: bool) -> impl FnMut(&str) -> IResult<&str, HandValue> {
    move |s: &str| {
        map(one_of("23456789TJQKA"), |c: char| {
            Reverse(match c {
                '2'..='9' => c as u8 - b'0',
                'T' => 10,
                'J' => {
                    if part2 {
                        1
                    } else {
                        11
                    }
                }
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("unknown symbol"),
            })
        })(s)
    }
}

fn parse_hand(p2: bool) -> impl FnMut(&str) -> IResult<&str, Vec<HandValue>> {
    move |s: &str| {
        map(
            tuple((
                parse_card(p2),
                parse_card(p2),
                parse_card(p2),
                parse_card(p2),
                parse_card(p2),
            )),
            |x| vec![x.0, x.1, x.2, x.3, x.4],
        )(s)
    }
}

fn parse_line(p2: bool) -> impl FnMut(&str) -> IResult<&str, Hand> {
    move |s: &str| {
        let (s, cards) = parse_hand(p2)(s)?;
        let (s, _) = space1(s)?;
        let (s, value) = nom_usize(s)?;
        Ok((s, Hand { cards, value }))
    }
}

fn generator<const JOKER: u8>(input: &str) -> Vec<Hand> {
    let mut v = process_input(nom_lines(parse_line(JOKER == 1)))(input);
    v.sort_by(|s, other| s.compare(other, JOKER));
    v
}

#[aoc_generator(day7, part1)]
pub fn generator_p1(input: &str) -> Vec<Hand> {
    generator::<0>(input)
}

#[aoc_generator(day7, part2)]
pub fn generator_p2(input: &str) -> Vec<Hand> {
    generator::<1>(input)
}

fn solve(inputs: &[Hand]) -> usize {
    inputs
        .iter()
        .enumerate()
        .map(|(place, hand)| hand.value * (place + 1))
        .sum()
}

#[aoc(day7, part1)]
pub fn part1(inputs: &[Hand]) -> usize {
    solve(inputs)
}

#[aoc(day7, part2)]
pub fn part2(inputs: &[Hand]) -> usize {
    solve(inputs)
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
        println!("{:?}", generator_p2(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator_p1(SAMPLE)), 6440);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator_p2(SAMPLE)), 5905);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day7.txt");
        const ANSWERS: (usize, usize) = (251058093, 249781879);

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
