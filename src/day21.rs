use std::{fmt::Debug, isize};

use ahash::{HashSet, HashSetExt};
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
    grid: Vec<Vec<State>>,
    width: isize,
    height: isize,
    start: (isize, isize),
}

impl Grid {
    fn get(&self, index: (isize, isize)) -> Option<&State> {
        let r = usize::try_from(index.0.rem_euclid(self.height)).unwrap();
        let c = usize::try_from(index.1.rem_euclid(self.width)).unwrap();

        self.grid.get(r).and_then(|row| row.get(c))
    }

    pub fn display(&self, locations: &HashSet<(isize, isize)>) {
        let ((min_y, min_x), (max_y, max_x)) = locations.iter().fold(
            ((isize::MAX, isize::MAX), (isize::MIN, isize::MIN)),
            |(min, max), loc| {
                (
                    (min.0.min(loc.0), min.1.min(loc.1)),
                    (max.0.max(loc.0), max.1.max(loc.1)),
                )
            },
        );

        for row in min_y..=max_y {
            for col in min_x..=max_x {
                if locations.contains(&(row, col)) {
                    print!("O");
                } else {
                    let c: &State = self.get((row, col)).unwrap();
                    print!("{:?}", c);
                }
            }
            println!();
        }

        println!();
        println!();
    }

    fn neighbors(&self, (r, c): (isize, isize)) -> [(isize, isize); 4] {
        [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]
    }

    fn step(&self, locations: &HashSet<(isize, isize)>) -> HashSet<(isize, isize)> {
        let mut res = HashSet::with_capacity(self.grid.len());

        for &index in locations.iter() {
            for neigh in self.neighbors(index) {
                if let Some(State::Plot) = self.get(neigh) {
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

        grid.push(
            l.bytes()
                .enumerate()
                .map(|(cidx, b)| match b {
                    b'#' => State::Rock,
                    b'.' => State::Plot,
                    b'S' => {
                        start = (height, cidx);
                        State::Plot
                    }
                    _ => panic!("unexpected byte"),
                })
                .collect(),
        );
        height += 1;
    }

    Grid {
        grid,
        height: isize::try_from(height).unwrap(),
        width: isize::try_from(width).unwrap(),
        start: (
            isize::try_from(start.0).unwrap(),
            isize::try_from(start.1).unwrap(),
        ),
    }
}

fn solve(inputs: &Grid, steps: usize) -> usize {
    let mut taken = HashSet::with_capacity(inputs.grid.len());
    taken.insert(inputs.start);

    for _ in 0..steps {
        taken = inputs.step(&taken);
        // inputs.display(&taken);
    }

    taken.len()
}

#[aoc(day21, part1)]
pub fn part1(inputs: &Grid) -> usize {
    solve(inputs, 64)
}

#[aoc(day21, part2)]
pub fn part2(inputs: &Grid) -> usize {
    let n = inputs.grid.len();
    let rem = 26_501_365 % n;

    let xs = [0, 1, 2];
    let mut ys = Vec::with_capacity(3);
    for i in [rem, rem + n, rem + n * 2] {
        let y = solve(inputs, i);
        ys.push(y as f64);
    }
    0
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
        let mut set = HashSet::new();
        set.insert(grid.start);
        grid.display(&set);

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(solve(&generator(SAMPLE), 6), 16);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(solve(&generator(SAMPLE), 6), 16);
        assert_eq!(solve(&generator(SAMPLE), 10), 50);
        assert_eq!(solve(&generator(SAMPLE), 50), 1594);
        assert_eq!(solve(&generator(SAMPLE), 100), 6536);
        assert_eq!(solve(&generator(SAMPLE), 500), 167004);

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
