use aoc_runner_derive::aoc;
use itertools::Itertools;

fn parse_data<const EXP: usize>(s: &str) -> Vec<(usize, usize)> {
    let grid = s
        .lines()
        .map(|line| line.bytes().collect_vec())
        .collect_vec();

    let mut input = s
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.bytes()
                .enumerate()
                .filter(|(_, b)| b == &b'#')
                .map(move |(col, _)| (row, col))
        })
        .collect_vec();

    let mut adjustment = 0;
    for (row, line) in s.lines().enumerate() {
        if line.bytes().all(|b| b == b'.') {
            for galaxy in input.iter_mut() {
                if galaxy.0 > row + adjustment {
                    galaxy.0 += EXP;
                }
            }
            adjustment += EXP;
        }
    }

    let mut adjustment = 0;
    for col in 0..grid[0].len() {
        let mut empty = true;
        for row in &grid {
            if row[col] == b'#' {
                empty = false;
                break;
            }
        }

        if empty {
            for galaxy in input.iter_mut() {
                if galaxy.1 > col + adjustment {
                    galaxy.1 += EXP;
                }
            }
            adjustment += EXP;
        }
    }

    input
}

fn solve<const EXP: usize>(s: &str) -> usize {
    parse_data::<EXP>(s)
        .into_iter()
        .permutations(2)
        // .inspect(|x| println!("{x:?}"))
        .map(|pairs| {
            assert_eq!(pairs.len(), 2);
            pairs[0].0.abs_diff(pairs[1].0) + pairs[0].1.abs_diff(pairs[1].1)
        })
        .sum::<usize>()
        / 2
}

#[aoc(day11, part1)]
pub fn part1(inputs: &str) -> usize {
    solve::<1>(inputs)
}

#[aoc(day11, part2)]
pub fn part2(inputs: &str) -> usize {
    solve::<999999>(inputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    pub fn input_test() {
        const RESULT: &str = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";

        let input: Vec<_> = RESULT
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.bytes()
                    .enumerate()
                    .filter(|(_, b)| b == &b'#')
                    .map(move |(col, _)| (row, col))
            })
            .collect();

        assert_eq!(parse_data::<1>(SAMPLE), input);
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(SAMPLE), 374);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(solve::<9>(SAMPLE), 1030);
        assert_eq!(solve::<99>(SAMPLE), 8410);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day11.txt");
        const ANSWERS: (usize, usize) = (10228230, 447073334102);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            // let output = generator(input);

            assert_eq!(part1(input), ANSWERS.0);
            assert_eq!(part2(input), ANSWERS.1);
        }
    }
}
