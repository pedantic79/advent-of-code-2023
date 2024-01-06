use std::{fmt::Debug, isize};

use ahash::{HashSet, HashSetExt};
use aoc_runner_derive::{aoc, aoc_generator};
use num::Integer;
use polyfit_rs::polyfit_rs::polyfit;

#[repr(u8)]
#[derive(PartialEq, Eq, Clone)]
pub enum State {
    Rock = b'#',
    Plot = b'.',
    Start = b'S',
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rock => write!(f, "#"),
            Self::Plot => write!(f, "."),
            Self::Start => write!(f, "S"),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Grid {
    grid: Vec<State>,
    width: isize,
    height: isize,
    start: (isize, isize),
}

impl Grid {
    fn get(&self, index: (isize, isize)) -> Option<&State> {
        let r = index.0.rem_euclid(self.height);
        let c = index.1.rem_euclid(self.width);

        self.grid.get(to_usize(r * (self.width + 1) + c))
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

    fn step(&self, input: &HashSet<(isize, isize)>, output: &mut HashSet<(isize, isize)>) {
        for &index in input.iter() {
            for neigh in self.neighbors(index) {
                if let Some(State::Plot | State::Start) = self.get(neigh) {
                    output.insert(neigh);
                }
            }
        }
    }
}

fn to_isize(u: usize) -> isize {
    isize::try_from(u).unwrap()
}

fn to_usize(i: isize) -> usize {
    usize::try_from(i).unwrap()
}

#[aoc_generator(day21)]
pub fn generator(input: &str) -> Grid {
    let width = input.find('\n').unwrap();
    let height = input.len() / width;

    let grid = unsafe { std::mem::transmute::<&[u8], &[State]>(input.as_bytes()) };

    Grid {
        grid: grid.to_vec(),
        height: to_isize(height),
        width: to_isize(width),
        start: (to_isize(height / 2), to_isize(width / 2)),
    }
}

fn solve(inputs: &Grid, steps: usize) -> usize {
    let mut odd = HashSet::new();
    let mut even = HashSet::new();

    odd.insert(inputs.start);

    for _ in 0..steps / 2 {
        inputs.step(&odd, &mut even);
        // inputs.display(&even);
        inputs.step(&even, &mut odd);
        // inputs.display(&odd);
    }
    if steps.is_odd() {
        inputs.step(&odd, &mut even);
        even.len()
    } else {
        odd.len()
    }
}

#[aoc(day21, part1)]
pub fn part1(inputs: &Grid) -> usize {
    solve(inputs, 64)
}

#[aoc(day21, part2)]
pub fn part2(inputs: &Grid) -> usize {
    let n = to_usize(inputs.width);
    let rem = 26_501_365 % n;

    let xs = [0.0, 1.0, 2.0];
    let mut ys = Vec::with_capacity(3);
    for i in [rem, rem + n, rem + n * 2] {
        let y = solve(inputs, i);
        ys.push(y as f64);
    }

    let coefficients = polyfit(&xs, &ys, 2).unwrap();
    let equation = |x: usize| {
        coefficients[2].round() as usize * x * x
            + coefficients[1].round() as usize * x
            + coefficients[0].round() as usize
    };

    equation(26_501_365 / n)
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
        // assert_eq!(solve(&generator(SAMPLE), 500), 167004);

        // assert_eq!(part2(&generator(SAMPLE)), 336);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day21.txt");
        const ANSWERS: (usize, usize) = (3764, 622926941971282);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
