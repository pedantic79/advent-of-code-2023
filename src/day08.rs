use ahash::HashMapExt;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::{tag, take},
    character::complete::newline,
    combinator::map,
    sequence::{delimited, tuple},
    IResult,
};
use num::Integer;
use rayon::iter::{ParallelBridge, ParallelIterator};
use rustc_hash::FxHashMap as HashMap;
use smallstr::SmallString;

use crate::common::nom::fold_separated_list0;

type String = SmallString<[u8; 3]>;

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

fn parse_node(s: &str) -> IResult<&str, (String, String, String)> {
    let (s, key) = map(take(3usize), String::from)(s)?;
    let (s, _) = tag(" = ")(s)?;
    let (s, (l, _, r)) = delimited(
        tag("("),
        tuple((
            map(take(3usize), String::from),
            tag(", "),
            map(take(3usize), String::from),
        )),
        tag(")"),
    )(s)?;

    Ok((s, (key, l, r)))
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> (Vec<Direction>, HashMap<String, (String, String)>) {
    let (instructions, network) = input.split_once("\n\n").unwrap();

    let ins = instructions
        .bytes()
        .map(|b| {
            if b == b'L' {
                Direction::Left
            } else {
                Direction::Right
            }
        })
        .collect();

    let (_, hm) = fold_separated_list0(newline, parse_node, HashMap::new, |mut hm, (key, l, r)| {
        hm.insert(key, (l, r));
        hm
    })(network)
    .unwrap();
    (ins, hm)
}

#[aoc(day8, part1)]
pub fn part1((ins, net): &(Vec<Direction>, HashMap<String, (String, String)>)) -> usize {
    solve("AAA", ins, net, false)
}

fn solve<'a>(
    mut location: &'a str,
    ins: &'a [Direction],
    net: &'a HashMap<String, (String, String)>,
    part2: bool,
) -> usize {
    let mut steps = 0;

    for ins in ins.iter().cycle() {
        if location == "ZZZ" || part2 && location.ends_with('Z') {
            break;
        }

        steps += 1;
        let new_loc = match ins {
            Direction::Left => net.get(location).unwrap().0.as_str(),
            Direction::Right => net.get(location).unwrap().1.as_str(),
        };

        location = new_loc;
    }

    steps
}

#[aoc(day8, part2)]
pub fn part2((ins, net): &(Vec<Direction>, HashMap<String, (String, String)>)) -> usize {
    net.keys()
        .par_bridge()
        .filter(|k| k.ends_with('A'))
        .map(|k| solve(k, ins, net, true))
        .reduce(|| 1, |l, x| l.lcm(&x))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const SAMPLE2: &str = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const SAMPLE3: &str = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE2)), 6);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE3)), 6);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day8.txt");
        const ANSWERS: (usize, usize) = (19783, 9177460370549);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
