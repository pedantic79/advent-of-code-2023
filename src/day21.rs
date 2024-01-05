use std::{fmt::Debug, iter::repeat};

use aoc_runner_derive::{aoc, aoc_generator};

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
    start: (usize, usize),
}

impl Grid {
    pub fn display(&self, locations: &[bool]) {
        let mut idx = 0;
        for (c, plots) in self.grid.iter().zip(locations.iter().chain(repeat(&false))) {
            if *plots {
                print!("O");
            } else {
                print!("{:?}", c);
            }
            idx += 1;
            if idx == self.width {
                println!();
                idx = 0;
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

    fn step(&self, locations: &[bool]) -> Vec<bool> {
        let mut res = vec![false; locations.len()];

        for (index, &loc) in locations.iter().enumerate() {
            if loc {
                for neigh in self.neighbors(index) {
                    if let Some(state) = self.grid.get(neigh) {
                        if state == &State::Plot {
                            res[neigh] = true;
                        }
                    }
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
        start,
    }
}

fn solve<const STEPS: usize>(inputs: &Grid) -> usize {
    let mut taken = vec![false; inputs.grid.len()];
    taken[inputs.start.0 * inputs.width + inputs.start.1] = true;

    for _ in 0..STEPS {
        taken = inputs.step(&taken);
    }

    taken.iter().filter(|x| **x).count()
}

#[aoc(day21, part1)]
pub fn part1(inputs: &Grid) -> usize {
    solve::<64>(inputs)
}

// #[aoc(day21, part2)]
pub fn part2(inputs: &Grid) -> usize {
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
        grid.display(&[]);

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
