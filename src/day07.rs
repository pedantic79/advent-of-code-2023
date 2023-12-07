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

fn parse_card_p2(s: &str) -> IResult<&str, HandValue> {
    map(one_of("23456789TJQKA"), |c: char| {
        Reverse(match c {
            '2'..='9' => c as u8 - b'0',
            'T' => 10,
            'J' => 1,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("unknown symbol"),
        })
    })(s)
}

fn parse_hand(s: &str) -> IResult<&str, Vec<HandValue>> {
    map(
        tuple((parse_card, parse_card, parse_card, parse_card, parse_card)),
        |x| vec![x.0, x.1, x.2, x.3, x.4],
    )(s)
}

fn parse_hand_p2(s: &str) -> IResult<&str, Vec<HandValue>> {
    map(
        tuple((
            parse_card_p2,
            parse_card_p2,
            parse_card_p2,
            parse_card_p2,
            parse_card_p2,
        )),
        |x| vec![x.0, x.1, x.2, x.3, x.4],
    )(s)
}

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
        other
            .classify()
            .cmp(&self.classify())
            .then_with(|| other.cards.cmp(&self.cards))
    }
}

impl Hand {
    fn classify(&self) -> HandTypes {
        let mut cards = self.cards.clone();
        cards.sort_by_key(|&x| x);
        let mut data_grouped: Vec<Vec<HandValue>> = Vec::new();

        for (_, group) in &cards.into_iter().group_by(|elt| *elt) {
            data_grouped.push(group.collect());
        }

        data_grouped.sort_by_key(|x| Reverse(x.len()));
        let cards = data_grouped;

        match cards[0].len() {
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

    fn classify_p2(&self) -> HandTypes {
        let mut cards: Vec<HandValue> = self.cards.iter().filter(|x| x.0 != 1).copied().collect();
        let jokers = self.cards.iter().filter(|x| x.0 == 1).count();

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
}

fn parse_line(s: &str) -> IResult<&str, Hand> {
    let (s, cards) = parse_hand(s)?;
    let (s, _) = space1(s)?;
    let (s, value) = nom_usize(s)?;
    Ok((s, Hand { cards, value }))
}

fn parse_line_p2(s: &str) -> IResult<&str, Hand> {
    let (s, cards) = parse_hand_p2(s)?;
    let (s, _) = space1(s)?;
    let (s, value) = nom_usize(s)?;
    Ok((s, Hand { cards, value }))
}

#[aoc_generator(day7, part1)]
pub fn generator(input: &str) -> Vec<Hand> {
    let mut v = process_input(nom_lines(parse_line))(input);
    v.sort();
    v
}

#[aoc_generator(day7, part2)]
pub fn generator_p2(input: &str) -> Vec<Hand> {
    let mut v = process_input(nom_lines(parse_line_p2))(input);
    v.sort_by(|s, other| {
        other
            .classify_p2()
            .cmp(&s.classify_p2())
            .then_with(|| other.cards.cmp(&s.cards))
    });
    v
}

#[aoc(day7, part1)]
pub fn part1(inputs: &[Hand]) -> usize {
    inputs
        .iter()
        .enumerate()
        .map(|(place, hand)| hand.value * (place + 1))
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(inputs: &[Hand]) -> usize {
    inputs
        .iter()
        .enumerate()
        .map(|(place, hand)| hand.value * (place + 1))
        .sum()
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
        // println!("{:?}", generator_p2(SAMPLE));

        for x in generator_p2(SAMPLE) {
            println!("{:?} {:?}", x, x.classify_p2())
        }

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
        const ANSWERS: (usize, usize) = (251058093, 249781879);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output1 = generator(input);
            let output2 = generator_p2(input);

            assert_eq!(part1(&output1), ANSWERS.0);
            assert_eq!(part2(&output2), ANSWERS.1);
        }
    }
}
