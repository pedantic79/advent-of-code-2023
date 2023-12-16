use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Object {
    Empty,
    Slash,
    BackSlash,
    Pipe,
    Dash,
}

impl Object {
    fn next_dir(&self, d: &Dir) -> Dir {
        match (self, d) {
            (Object::BackSlash, Dir::North) => Dir::West,
            (Object::BackSlash, Dir::West) => Dir::North,
            (Object::BackSlash, Dir::South) => Dir::East,
            (Object::BackSlash, Dir::East) => Dir::South,
            (Object::Slash, Dir::North) => Dir::East,
            (Object::Slash, Dir::West) => Dir::South,
            (Object::Slash, Dir::South) => Dir::West,
            (Object::Slash, Dir::East) => Dir::North,
            _ => unreachable!("don't pass a non-mirror"),
        }
    }

    fn next_split(&self, d: Dir, pos: (usize, usize)) -> [Option<((usize, usize), Dir)>; 2] {
        match (self, d) {
            (Object::Pipe, Dir::North | Dir::South) => [Some((d.next_pos(pos), d)), None],
            (Object::Dash, Dir::West | Dir::East) => [Some((d.next_pos(pos), d)), None],
            (Object::Pipe, Dir::West | Dir::East) => [
                Some((Dir::North.next_pos(pos), Dir::North)),
                Some((Dir::South.next_pos(pos), Dir::South)),
            ],
            (Object::Dash, Dir::North | Dir::South) => [
                Some((Dir::West.next_pos(pos), Dir::West)),
                Some((Dir::East.next_pos(pos), Dir::East)),
            ],
            _ => unreachable!("don't pass a non-splitter"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Dir {
    North,
    West,
    South,
    East,
}

impl Dir {
    fn next_pos(&self, (y, x): (usize, usize)) -> (usize, usize) {
        match self {
            Dir::North => (y.wrapping_sub(1), x),
            Dir::West => (y, x.wrapping_sub(1)),
            Dir::South => (y + 1, x),
            Dir::East => (y, x + 1),
        }
    }

    fn get_mask(&self) -> u8 {
        match self {
            Dir::North => 0b0001,
            Dir::West => 0b0010,
            Dir::South => 0b0100,
            Dir::East => 0b1000,
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

fn solve(inputs: &[Vec<Object>], start: ((usize, usize), Dir)) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = vec![vec![0; inputs[0].len()]; inputs.len()];

    queue.push_back(start);

    while let Some((pos, dir)) = queue.pop_front() {
        if let Some(kind) = inputs.get(pos.0).and_then(|row| row.get(pos.1)) {
            if seen[pos.0][pos.1] & dir.get_mask() > 0 {
                continue;
            }

            seen[pos.0][pos.1] |= dir.get_mask();
            match kind {
                Object::Empty => queue.push_back((dir.next_pos(pos), dir)),
                Object::Slash | Object::BackSlash => {
                    let new_dir = kind.next_dir(&dir);
                    queue.push_back((new_dir.next_pos(pos), new_dir));
                }
                Object::Pipe | Object::Dash => {
                    for next in kind.next_split(dir, pos).into_iter().flatten() {
                        queue.push_back(next);
                    }
                }
            }
        }
    }

    seen.iter()
        .map(|row| row.iter().filter(|&&x| x > 0).count())
        .sum()
}

#[aoc(day16, part1)]
pub fn part1(inputs: &[Vec<Object>]) -> usize {
    solve(inputs, ((0, 0), Dir::East))
}

#[aoc(day16, part2)]
pub fn part2(inputs: &Vec<Vec<Object>>) -> usize {
    let height = inputs.len();
    let width = inputs[0].len();

    (0..height)
        .map(|r| ((r, 0), Dir::East))
        .chain((0..height).map(|r| ((r, width - 1), Dir::West)))
        .chain((0..width).map(|c| ((0, c), Dir::South)))
        .chain((0..width).map(|c| ((height - 1, c), Dir::North)))
        .par_bridge()
        .map(|x| solve(inputs, x))
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

        println!("{}", solve(&generator(SAMPLE), ((0, 3), Dir::South)));

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
