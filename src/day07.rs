use std::cmp::{Ordering, Reverse};

use aoc_runner_derive::{aoc, aoc_generator};
use arrayvec::ArrayVec;
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

#[derive(Debug, Clone)]
pub struct Hand {
    cards: [HandValue; 5],
    value: usize,
}

impl Hand {
    fn classify(&self, joker: u8) -> HandTypes {
        let jokers = self.cards.iter().filter(|x| x.0 == joker).count();
        let mut cards: ArrayVec<HandValue, 5> = self
            .cards
            .iter()
            .filter(|x| x.0 != joker)
            .copied()
            .collect();

        cards.sort_unstable_by_key(|&x| x);
        let mut data_grouped: ArrayVec<ArrayVec<HandValue, 5>, 5> = ArrayVec::new();

        for (_, group) in &cards.into_iter().group_by(|elt| *elt) {
            data_grouped.push(group.collect());
        }

        data_grouped.sort_unstable_by_key(|x| Reverse(x.len()));
        let cards = data_grouped;

        if cards.is_empty() {
            // all hands are jokers
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

    fn cards_compare(s: &[HandValue], other: &[HandValue], joker: bool) -> Ordering {
        for (a, b) in s.iter().zip(other.iter()) {
            let a = if joker && a.0 == 11 { Reverse(1) } else { *a };
            let b = if joker && b.0 == 11 { Reverse(1) } else { *b };

            let c = a.cmp(&b);
            if c != Ordering::Equal {
                return c;
            }
        }
        Ordering::Equal
    }

    fn compare(&self, other: &Self, joker: u8) -> Ordering {
        other
            .classify(joker)
            .cmp(&self.classify(joker))
            .then_with(|| Self::cards_compare(&other.cards, &self.cards, joker == 11))
    }
}

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

fn parse_hand(s: &str) -> IResult<&str, [HandValue; 5]> {
    map(
        tuple((parse_card, parse_card, parse_card, parse_card, parse_card)),
        |x| [x.0, x.1, x.2, x.3, x.4],
    )(s)
}

fn parse_line(s: &str) -> IResult<&str, Hand> {
    let (s, cards) = parse_hand(s)?;
    let (s, _) = space1(s)?;
    let (s, value) = nom_usize(s)?;
    Ok((s, Hand { cards, value }))
}

#[aoc_generator(day7)]
fn generator(input: &str) -> Vec<Hand> {
    process_input(nom_lines(parse_line))(input)
}

fn solve<const JOKER: u8>(inputs: &[Hand]) -> usize {
    let mut inputs = inputs.to_vec();
    inputs.sort_unstable_by(|a, b| a.compare(b, JOKER));

    inputs
        .into_iter()
        .enumerate()
        // .inspect(|x| println!("{x:?} {:?}", x.1.classify(JOKER)))
        .map(|(place, hand)| hand.value * (place + 1))
        .sum()
}

#[aoc(day7, part1)]
pub fn part1(inputs: &[Hand]) -> usize {
    solve::<0>(inputs)
}

#[aoc(day7, part2)]
pub fn part2(inputs: &[Hand]) -> usize {
    solve::<11>(inputs)
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
        assert_eq!(part2(&generator(SAMPLE)), 5905);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day7.txt");
        const ANSWERS: (usize, usize) = (251058093, 249781879);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
