use ahash::HashMapExt;
use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashMap as HashMap;

use crate::common::matrix::rotate_right;
const TARGET: usize = 1_000_000_000;

#[derive(PartialEq, Eq, Hash)]
pub struct Dish {
    dish: Vec<Vec<u8>>,
}

impl std::fmt::Debug for Dish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.dish {
            writeln!(f, "{}", String::from_utf8_lossy(row))?;
        }

        Ok(())
    }
}

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Dish {
    Dish {
        dish: input.lines().map(|line| line.bytes().collect()).collect(),
    }
}

fn roll(grid: &mut Vec<Vec<u8>>) {
    for c in 0..grid[0].len() {
        for _ in 0..grid.len() {
            for r in 0..grid.len() {
                if grid[r][c] == b'O' && r > 0 && grid[r - 1][c] == b'.' {
                    grid[r][c] = b'.';
                    grid[r - 1][c] = b'O';
                }
            }
        }
    }
}

fn cycle(grid: &mut Vec<Vec<u8>>) {
    for _ in 0..4 {
        roll(grid);
        rotate_right(grid);
    }
}

fn load(grid: &Vec<Vec<u8>>) -> usize {
    grid.iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .filter_map(|&cell| (cell == b'O').then_some(grid.len() - r))
                .sum::<usize>()
        })
        .sum()
}

#[aoc(day14, part1)]
pub fn part1(platform: &Dish) -> usize {
    let mut dish = platform.dish.to_vec();

    roll(&mut dish);
    load(&dish)
}

#[aoc(day14, part2)]
pub fn part2(platform: &Dish) -> usize {
    let mut dish = platform.dish.to_vec();
    let mut seen = HashMap::new();
    seen.insert(dish.clone(), 0);

    let mut t = 0;
    while t < TARGET {
        t += 1;
        cycle(&mut dish);

        if let Some(old) = seen.get(&dish) {
            let cycle = t - old;
            let amt = (TARGET - t) / cycle;
            t += amt * cycle;
        }

        seen.insert(dish.clone(), t);
    }

    load(&dish)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    pub fn input_test() {
        let mut platform = generator(SAMPLE);
        println!("{:?}", platform);

        // push_left(&mut platform.dish[9]);

        // for i in 0..platform.dish.len() {
        //     push_left(&mut platform.dish[i]);
        // }
        println!("{:?}", platform);

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 136);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 64);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day14.txt");
        const ANSWERS: (usize, usize) = (109385, 93102);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            // assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
