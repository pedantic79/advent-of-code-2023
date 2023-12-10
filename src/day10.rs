use ahash::{HashSet, HashSetExt};
use aoc_runner_derive::{aoc, aoc_generator};

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
}

fn next_direction(pipe: u8, dir: &Dir) -> Option<Dir> {
    Some(match (pipe, dir) {
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

#[aoc_generator(day10)]
pub fn generator(input: &str) -> (HashSet<(usize, usize)>, Vec<Vec<u8>>) {
    let mut start = (0, 0);

    let grid: Vec<_> = input
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

    (solve(&grid, start), grid)
}

fn solve(grid: &[Vec<u8>], start: (usize, usize)) -> HashSet<(usize, usize)> {
    let (mut r, mut c) = start;
    let mut dir = Dir::Down; // assume we can go down first
    let mut pipe_set = HashSet::new();
    pipe_set.insert((r, c));

    loop {
        dir.next_pos(&mut r, &mut c);
        let pipe = grid[r][c];
        if pipe == b'S' {
            break;
        }
        pipe_set.insert((r, c));

        dir = next_direction(pipe, &dir)
            .unwrap_or_else(|| panic!("Unknown pipe combination {} {:?}", char::from(pipe), dir,));
    }

    pipe_set
}

#[aoc(day10, part1)]
pub fn part1((maze, _): &(HashSet<(usize, usize)>, Vec<Vec<u8>>)) -> usize {
    maze.len() / 2
}

#[aoc(day10, part2)]
pub fn part2((maze, grid): &(HashSet<(usize, usize)>, Vec<Vec<u8>>)) -> usize {
    let mut count = 0;
    for (row, line) in grid.iter().enumerate() {
        // we can as assume we're outside by default
        let mut inside = false;
        for (col, cell) in line.iter().enumerate() {
            if maze.contains(&(row, col)) {
                // Check what parity we are in.
                // If we see a vertical, then we go outside to inside and vice-versa
                if b"|JLS".contains(cell) {
                    inside = !inside;
                }
            } else {
                count += usize::from(inside);
            }
        }
    }
    count
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
