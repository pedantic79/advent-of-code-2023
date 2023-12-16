use std::collections::VecDeque;

use ahash::HashSetExt;
use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rustc_hash::FxHashSet as HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Object {
    Empty,
    Slash,
    BackSlash,
    Pipe,
    Dash,
}

fn up((y, x): (usize, usize)) -> (usize, usize) {
    (y.wrapping_sub(1), x)
}

fn left((y, x): (usize, usize)) -> (usize, usize) {
    (y, x.wrapping_sub(1))
}

impl Object {
    fn next_dir(&self, d: &Direction) -> Direction {
        match (self, d) {
            (Object::BackSlash, Direction::North) => Direction::West,
            (Object::BackSlash, Direction::West) => Direction::North,
            (Object::BackSlash, Direction::South) => Direction::East,
            (Object::BackSlash, Direction::East) => Direction::South,
            (Object::Slash, Direction::North) => Direction::East,
            (Object::Slash, Direction::West) => Direction::South,
            (Object::Slash, Direction::South) => Direction::West,
            (Object::Slash, Direction::East) => Direction::North,
            _ => unreachable!("don't pass a non-mirror"),
        }
    }

    fn next_split(
        &self,
        d: &Direction,
        (y, x): (usize, usize),
    ) -> [Option<((usize, usize), Direction)>; 2] {
        match (self, d) {
            (Object::Pipe, Direction::North) => {
                [Some((d.next_pos((y, x)), Direction::North)), None]
            }
            (Object::Pipe, Direction::South) => {
                [Some((d.next_pos((y, x)), Direction::South)), None]
            }
            (Object::Dash, Direction::West) => [Some((d.next_pos((y, x)), Direction::West)), None],
            (Object::Dash, Direction::East) => [Some((d.next_pos((y, x)), Direction::East)), None],
            (Object::Pipe, Direction::West | Direction::East) => [
                Some((up((y, x)), Direction::North)),
                Some(((y + 1, x), Direction::South)),
            ],
            (Object::Dash, Direction::North | Direction::South) => [
                Some((left((y, x)), Direction::West)),
                Some(((y, x + 1), Direction::East)),
            ],
            _ => unreachable!("don't pass a non-splitter"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn next_pos(&self, (y, x): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => up((y, x)),
            Direction::West => left((y, x)),
            Direction::South => (y + 1, x),
            Direction::East => (y, x + 1),
        }
    }
}

#[aoc_generator(day16)]
pub fn generator(input: &str) -> Vec<Vec<Object>> {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|cell| match cell {
                    b'.' => Object::Empty,
                    b'/' => Object::Slash,
                    b'\\' => Object::BackSlash,
                    b'|' => Object::Pipe,
                    b'-' => Object::Dash,
                    _ => panic!("unexpected character"),
                })
                .collect()
        })
        .collect()
}

fn solve(inputs: &[Vec<Object>], start: ((usize, usize), Direction)) -> usize {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some((pos, dir)) = queue.pop_front() {
        if let Some(kind) = inputs.get(pos.0).and_then(|row| row.get(pos.1)) {
            if seen.contains(&(pos, dir)) {
                continue;
            }
            seen.insert((pos, dir));
            match kind {
                Object::Empty => queue.push_back((dir.next_pos(pos), dir)),
                Object::Slash | Object::BackSlash => {
                    let new_dir = kind.next_dir(&dir);
                    queue.push_back((new_dir.next_pos(pos), new_dir));
                }
                Object::Pipe => {
                    let [u, d] = kind.next_split(&dir, pos);
                    if let Some(u) = u {
                        queue.push_back(u);
                    }

                    if let Some(d) = d {
                        queue.push_back(d);
                    }
                }
                Object::Dash => {
                    let [l, r] = kind.next_split(&dir, pos);
                    if let Some(l) = l {
                        queue.push_back(l);
                    }

                    if let Some(r) = r {
                        queue.push_back(r);
                    }
                }
            }
        }
    }

    let seen: HashSet<_> = seen.into_iter().map(|x| x.0).collect();
    seen.len()
}

#[aoc(day16, part1)]
pub fn part1(inputs: &[Vec<Object>]) -> usize {
    solve(inputs, ((0, 0), Direction::East))
}

#[aoc(day16, part2)]
pub fn part2(inputs: &Vec<Vec<Object>>) -> usize {
    let height = inputs.len();
    let width = inputs[0].len();

    (0..height)
        .into_par_iter()
        .map(|r| solve(inputs, ((r, 0), Direction::East)))
        .chain(
            (0..height)
                .into_par_iter()
                .map(|r| solve(inputs, ((r, width - 1), Direction::West))),
        )
        .chain(
            (0..width)
                .into_par_iter()
                .map(|c| solve(inputs, ((0, c), Direction::South))),
        )
        .chain(
            (0..width)
                .into_par_iter()
                .map(|c| solve(inputs, ((height - 1, c), Direction::North))),
        )
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        println!("{}", solve(&generator(SAMPLE), ((0, 3), Direction::South)));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 46);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 51);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day16.txt");
        const ANSWERS: (usize, usize) = (8389, 8564);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
