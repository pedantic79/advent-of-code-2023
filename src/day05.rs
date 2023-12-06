use std::{
    cmp::{max, min},
    ops::Range,
};

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::space1,
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use crate::common::nom::{nom_lines, nom_usize, process_input};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IndividualMapper {
    source: Range<usize>,
    destination: usize,
}

impl IndividualMapper {
    fn contains(&self, input: usize) -> Option<usize> {
        self.source
            .contains(&input)
            .then_some(self.destination + input.saturating_sub(self.source.start))
    }
}

#[derive(Debug, Clone)]
pub struct GroupMapper {
    mappers: Vec<IndividualMapper>,
}

impl GroupMapper {
    fn apply_range(
        &self,
        input: &mut Vec<Range<usize>>,
        temp: &mut Vec<Range<usize>>,
        output: &mut Vec<Range<usize>>,
    ) {
        for mapper in &self.mappers {
            while let Some(range) = input.pop() {
                let before = range.start..min(range.end, mapper.source.start);
                let inter = (
                    max(range.start, mapper.source.start),
                    min(mapper.source.end, range.end),
                );
                let after = max(mapper.source.end, range.start)..range.end;

                if before.end > before.start {
                    temp.push(before);
                }

                if inter.1 > inter.0 {
                    output.push(
                        (inter.0 - mapper.source.start + mapper.destination)
                            ..(inter.1 - mapper.source.start + mapper.destination),
                    );
                }

                if after.end > after.start {
                    temp.push(after)
                }
            }
            std::mem::swap(input, temp);
        }

        output.append(input);
    }
}

fn parse_mapper(s: &str) -> IResult<&str, IndividualMapper> {
    map(
        tuple((nom_usize, space1, nom_usize, space1, nom_usize)),
        |(a, _, b, _, c)| IndividualMapper {
            source: b..(b + c),
            destination: a,
        },
    )(s)
}

fn parse(s: &str) -> IResult<&str, GroupMapper> {
    let (s, _) = take_until("\n")(s)?;
    let (s, _) = tag("\n")(s)?;
    map(nom_lines(parse_mapper), |mappers| GroupMapper { mappers })(s)
}

fn parse_seed(s: &str) -> IResult<&str, Vec<usize>> {
    let (s, _) = take_until(": ")(s)?;
    let (s, _) = tag(": ")(s)?;
    separated_list1(space1, nom_usize)(s)
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> (Vec<usize>, Vec<GroupMapper>) {
    let mut groups = input.split("\n\n");

    let seeds = process_input(parse_seed)(groups.next().unwrap());
    let gm = groups.map(process_input(parse)).collect();

    (seeds, gm)
}

fn solve(seeds: impl IntoIterator<Item = usize>, mappers: &[GroupMapper]) -> usize {
    seeds
        .into_iter()
        .map(|seed| {
            mappers.iter().fold(seed, |seed, gm| {
                gm.mappers
                    .iter()
                    .find_map(|mapper| mapper.contains(seed))
                    .unwrap_or(seed)
            })
        })
        .min()
        .unwrap()
}

#[aoc(day5, part1)]
pub fn part1((seeds, mappers): &(Vec<usize>, Vec<GroupMapper>)) -> usize {
    solve(seeds.iter().copied(), mappers)
}

#[aoc(day5, part2)]
pub fn part2((seeds_range, mappers): &(Vec<usize>, Vec<GroupMapper>)) -> usize {
    let mut temp = vec![];
    let mut output = vec![];
    let mut input = vec![];

    seeds_range
        .chunks(2)
        .map(|pair| {
            input.push(pair[0]..(pair[0] + pair[1]));
            for m in mappers {
                m.apply_range(&mut input, &mut temp, &mut output);
                std::mem::swap(&mut input, &mut output);
            }

            let min = input.iter().map(|x| x.start).min().unwrap();
            input.clear();
            min
        })
        .min()
        .unwrap()

    // seeds_range
    //     .par_chunks(2)
    //     .map(|x| solve(x[0]..(x[0] + x[1]), mappers))
    //     .min()
    //     .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn mapper() {
        let mapper = IndividualMapper {
            source: 50..(50 + 48),
            destination: 52,
        };

        assert_eq!(mapper.contains(79), Some(81));
        assert_eq!(mapper.contains(14), None);
        assert_eq!(mapper.contains(55), Some(57));
        assert_eq!(mapper.contains(13), None);
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 35);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 46);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day5.txt");
        const ANSWERS: (usize, usize) = (199602917, 2254686);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
