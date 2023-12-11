use ahash::HashSetExt;
use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rustc_hash::FxHashSet as HashSet;

#[derive(Debug)]
enum Dir {
    Down,
    Left,
    Up,
    Right,
}

impl Dir {
    fn next_pos(&self, r: &mut usize, c: &mut usize) {
        match self {
            Dir::Down => *r += 1,
            Dir::Left => *c -= 1,
            Dir::Up => *r -= 1,
            Dir::Right => *c += 1,
        }
    }

    fn next_direction(&self, pipe: u8) -> Option<Self> {
        Some(match (pipe, self) {
            (b'|', Dir::Down) => Dir::Down,
            (b'|', Dir::Up) => Dir::Up,
            (b'-', Dir::Left) => Dir::Left,
            (b'-', Dir::Right) => Dir::Right,
            (b'L', Dir::Down) => Dir::Right,
            (b'L', Dir::Left) => Dir::Up,
            (b'J', Dir::Down) => Dir::Left,
            (b'J', Dir::Right) => Dir::Up,
            (b'7', Dir::Right) => Dir::Down,
            (b'7', Dir::Up) => Dir::Left,
            (b'F', Dir::Up) => Dir::Right,
            (b'F', Dir::Left) => Dir::Down,
            _ => return None,
        })
    }
}

#[derive(Debug)]
pub struct Maze {
    pipe_loop: HashSet<(usize, usize)>,
    grid: Vec<Vec<u8>>,
}

impl Maze {
    fn new(grid: Vec<Vec<u8>>, start: (usize, usize), mut dir: Dir) -> Self {
        let (mut r, mut c) = start;
        let mut pipe_loop = HashSet::new();
        pipe_loop.insert((r, c));

        loop {
            dir.next_pos(&mut r, &mut c);
            if (r, c) == start {
                break;
            }
            pipe_loop.insert((r, c));
            dir = dir.next_direction(grid[r][c]).unwrap_or_else(|| {
                panic!(
                    "Unknown pipe combination {} {:?}",
                    char::from(grid[r][c]),
                    dir,
                )
            });
        }

        Self { pipe_loop, grid }
    }
}

fn get(grid: &[Vec<u8>], r: usize, c: usize) -> &u8 {
    grid.get(r).and_then(|x| x.get(c)).unwrap_or(&b'.')
}

fn determine_start(grid: &[Vec<u8>], r: usize, c: usize) -> (u8, Dir) {
    let up = b"|7F".contains(get(grid, r.wrapping_sub(1), c));
    let down = b"|LJ".contains(get(grid, r + 1, c));
    let left = b"-FL".contains(get(grid, r, c.wrapping_sub(1)));
    let right = b"-J7".contains(get(grid, r, c + 1));

    match (up, down, left, right) {
        (false, false, true, true) => (b'-', Dir::Right),
        (false, true, false, true) => (b'F', Dir::Down),
        (false, true, true, false) => (b'7', Dir::Down),
        (true, false, false, true) => (b'L', Dir::Right),
        (true, false, true, false) => (b'J', Dir::Up),
        (true, true, false, false) => (b'|', Dir::Down),
        _ => panic!("unknown piece {up} {down} {left} {right}"),
    }
}

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Maze {
    let mut start: (usize, usize) = (0, 0);

    let mut grid: Vec<Vec<u8>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.bytes()
                .enumerate()
                .map(|(col, cell)| {
                    if cell == b'S' {
                        start = (row, col)
                    }
                    cell
                })
                .collect()
        })
        .collect();

    let (start_pipe, start_dir) = determine_start(&grid, start.0, start.1);
    grid[start.0][start.1] = start_pipe;

    Maze::new(grid, start, start_dir)
}

#[aoc(day10, part1)]
pub fn part1(maze: &Maze) -> usize {
    maze.pipe_loop.len() / 2
}

#[aoc(day10, part2)]
pub fn part2(maze: &Maze) -> usize {
    // scan row by row, checking to see if we are in or out of the loop
    maze.grid
        .par_iter()
        .enumerate()
        .map(|(row, line)| {
            // we are on the left of the first column, so we can assume we're outside
            let mut inside = false;
            let mut count = 0;

            for (col, cell) in line.iter().enumerate() {
                if maze.pipe_loop.contains(&(row, col)) {
                    // If we see a vertical, then we flip our state
                    // a "vertical" is |JL or |7F. We have to be consistent between
                    // JL or 7F on which we consider "vertical" but either JL xor 7F work
                    if b"|JL".contains(cell) {
                        inside = !inside;
                    }
                } else {
                    // if we aren't part of the loop, then we just add the bool to our count
                    // if we're inside we'll add 1, if we are outside 0
                    count += usize::from(inside);
                }
            }

            count
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    const SAMPLE2: &str = r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 8);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE2)), 10);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day10.txt");
        const ANSWERS: (usize, usize) = (6640, 411);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
