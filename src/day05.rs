use std::ops::Range;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::common::{range_intersect, utils::parse_split};

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
                let [before, inter, after] = range_intersect(range, &mapper.source);

                if let Some(before) = before {
                    temp.push(before);
                }

                if let Some(inter) = inter {
                    output.push(
                        (inter.start - mapper.source.start + mapper.destination)
                            ..(inter.end - mapper.source.start + mapper.destination),
                    );
                }

                if let Some(after) = after {
                    temp.push(after);
                }
            }
            std::mem::swap(input, temp);
        }

        // ranges that don't map
        output.append(input);
    }
}

fn parse(group: &str) -> GroupMapper {
    let mut mappers = vec![];
    for line in group.lines().skip(1) {
        let (a, b, c) = line
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        let source = b..(b + c);

        mappers.push(IndividualMapper {
            source,
            destination: a,
        });
    }
    GroupMapper { mappers }
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> (Vec<usize>, Vec<GroupMapper>) {
    let mut groups = input.split("\n\n");

    let (_, seeds_s) = groups.next().unwrap().split_once(':').unwrap();
    let seeds = parse_split(seeds_s.trim(), ' ');

    let gm = groups.map(parse).collect();

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
