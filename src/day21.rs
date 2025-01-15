use std::fmt::Debug;

use ahash::HashSetExt;
use aoc_runner_derive::{aoc, aoc_generator};
use num::Integer;
use polyfit_rs::polyfit_rs::polyfit;
use rustc_hash::FxHashSet as HashSet;

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

    fn step(
        &self,
        input: &mut Vec<(isize, isize)>,
        output: &mut HashSet<(isize, isize)>,
        temp: &mut Vec<(isize, isize)>,
    ) {
        for &index in input.iter() {
            for neigh in self.neighbors(index) {
                if let Some(State::Plot | State::Start) = self.get(neigh) {
                    if output.insert(neigh) {
                        temp.push(neigh);
                    }
                }
            }
        }
        input.clear();
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
    let start = input.find('S').unwrap();
    let start = (start / (width + 1), start % (width + 1));

    let grid = unsafe { std::mem::transmute::<&[u8], &[State]>(input.as_bytes()) };

    Grid {
        grid: grid.to_vec(),
        height: to_isize(height),
        width: to_isize(width),
        start: (to_isize(start.0), to_isize(start.1)),
    }
}

fn solve(inputs: &Grid, steps: usize) -> usize {
    solve_multiple(inputs, steps, &[steps])[0]
}

fn solve_multiple(inputs: &Grid, steps: usize, target_steps: &[usize]) -> Vec<usize> {
    let mut even_output = HashSet::new();
    let mut odd_output = HashSet::new();

    let mut frontier_odd = vec![inputs.start];
    let mut frontier_even = vec![];

    let mut pos = 0;
    let mut output = Vec::with_capacity(target_steps.len());

    for i in 0..steps / 2 {
        inputs.step(&mut frontier_odd, &mut odd_output, &mut frontier_even);
        if i * 2 + 1 == target_steps[pos] {
            pos += 1;
            output.push(odd_output.len());
        }
        // inputs.display(&even);

        inputs.step(&mut frontier_even, &mut even_output, &mut frontier_odd);
        if i * 2 + 2 == target_steps[pos] {
            pos += 1;
            output.push(even_output.len());
        }
        // inputs.display(&odd);
    }
    if steps.is_odd() {
        inputs.step(&mut frontier_odd, &mut odd_output, &mut frontier_even);
        if steps == target_steps[pos] {
            output.push(odd_output.len());
        }
    }

    output
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
    let ixs = [rem, rem + n, rem + n * 2];
    let iys = solve_multiple(inputs, ixs[ixs.len() - 1], &ixs);
    let ys: Vec<_> = iys.into_iter().map(|y| y as f64).collect();

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
