use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::{iter::ParallelIterator, slice::ParallelSlice};

use crate::common::utils::parse_split;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IndividualMapper {
    source: [usize; 2],
    destination: [usize; 2],
}

impl IndividualMapper {
    fn contains(&self, input: usize) -> Option<usize> {
        if self.source[0] <= input && input < self.source[1] {
            let diff = input - self.source[0];
            Some(self.destination[0] + diff)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct GroupMapper {
    mappers: Vec<IndividualMapper>,
}

fn parse(group: &str) -> GroupMapper {
    let mut mappers = vec![];
    for line in group.lines().skip(1) {
        let (a, b, c) = line
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        mappers.push(IndividualMapper {
            source: [b, b + c],
            destination: [a, a + c],
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

#[aoc(day5, part1)]
pub fn part1((seeds, mappers): &(Vec<usize>, Vec<GroupMapper>)) -> usize {
    solve(seeds.iter().copied(), mappers)
}

fn solve(seeds: impl IntoIterator<Item = usize>, mappers: &Vec<GroupMapper>) -> usize {
    seeds
        .into_iter()
        .map(|seed| {
            let mut seed = seed;
            for gm in mappers {
                seed = gm
                    .mappers
                    .iter()
                    .find_map(|mapper| mapper.contains(seed))
                    .unwrap_or(seed);
            }
            seed
        })
        .min()
        .unwrap()
}
#[aoc(day5, part2)]
pub fn part2((seeds_range, mappers): &(Vec<usize>, Vec<GroupMapper>)) -> usize {
    seeds_range
        .par_chunks(2)
        .map(|x| solve(x[0]..(x[0] + x[1]), mappers))
        .min()
        .unwrap()
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
            source: [50, 50 + 48],
            destination: [52, 52 + 48],
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
            // assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
