use std::fmt::Debug;

use aoc_runner_derive::{aoc, aoc_generator};
use bit_set::BitSet;

#[derive(PartialEq, Eq)]
pub enum State {
    Rock,
    Plot,
    Step,
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rock => write!(f, "#"),
            Self::Plot => write!(f, "."),
            Self::Step => write!(f, "O"),
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct Grid {
    grid: Vec<State>,
    width: usize,
    height: usize,
    start: usize,
}

impl Grid {
    pub fn display(&self, locations: &BitSet) {
        let mut target = self.width - 1;

        for (idx, c) in self.grid.iter().enumerate() {
            if locations.contains(idx) {
                print!("O");
            } else {
                print!("{:?}", c);
            }
            if idx == target {
                target += self.width;
                println!();
            }
        }
    }

    fn neighbors(&self, index: usize) -> [usize; 4] {
        [
            index + 1,
            index.saturating_sub(1),
            index + self.width,
            index.saturating_sub(self.width),
        ]
    }

    fn step(&self, locations: &BitSet) -> BitSet {
        let mut res = BitSet::with_capacity(self.grid.len());

        for index in locations.iter() {
            for neigh in self.neighbors(index) {
                if let Some(State::Plot) = self.grid.get(neigh) {
                    res.insert(neigh);
                }
            }
        }

        res
    }
}

#[aoc_generator(day21)]
pub fn generator(input: &str) -> Grid {
    let mut start = (0, 0);
    let mut height = 0;
    let mut width = 0;
    let mut grid = Vec::new();

    for l in input.lines() {
        width = l.len();

        grid.extend(l.bytes().enumerate().map(|(cidx, b)| match b {
            b'#' => State::Rock,
            b'.' => State::Plot,
            b'S' => {
                start = (height, cidx);
                State::Plot
            }
            _ => panic!("unexpected byte"),
        }));
        height += 1;
    }

    Grid {
        grid,
        height,
        width,
        start: start.0 * width + start.1,
    }
}

fn solve<const STEPS: usize>(inputs: &Grid) -> usize {
    let mut taken = BitSet::with_capacity(inputs.grid.len());
    taken.insert(inputs.start);

    for _ in 0..STEPS {
        taken = inputs.step(&taken);
    }

    taken.len()
}

#[aoc(day21, part1)]
pub fn part1(inputs: &Grid) -> usize {
    solve::<64>(inputs)
}

// #[aoc(day21, part2)]
pub fn part2(_inputs: &Grid) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    pub fn input_test() {
        // println!("{:?}", generator(SAMPLE));
        let grid = generator(SAMPLE);
        let mut set = BitSet::new();
        set.insert(grid.start);
        grid.display(&set);

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(solve::<6>(&generator(SAMPLE)), 16);
    }

    #[test]
    pub fn part2_test() {
        // assert_eq!(part2(&generator(SAMPLE)), 336);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day21.txt");
        const ANSWERS: (usize, usize) = (3764, 0);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            // assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
