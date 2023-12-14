use ahash::HashMapExt;
use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashMap as HashMap;

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

fn roll_north(grid: &mut Vec<Vec<u8>>) {
    for c in 0..grid[0].len() {
        let mut last: Option<usize> = None;
        for r in 0..grid.len() {
            if grid[r][c] == b'O' {
                if let Some(new_row) = last {
                    grid[r][c] = b'.';
                    grid[new_row][c] = b'O';
                    last = Some(new_row + 1);
                } else if let Some(new_row) =
                    (0..r).rev().take_while(|&x| grid[x][c] == b'.').last()
                {
                    grid[r][c] = b'.';
                    grid[new_row][c] = b'O';
                    last = Some(new_row + 1);
                }
            } else {
                last = None;
            }
        }
    }
}

fn roll_west(grid: &mut Vec<Vec<u8>>) {
    for r in 0..grid.len() {
        let mut last: Option<usize> = None;

        for c in 0..grid[0].len() {
            if grid[r][c] == b'O' {
                if let Some(new_col) = last {
                    grid[r][c] = b'.';
                    grid[r][new_col] = b'O';
                    last = Some(new_col + 1);
                } else if let Some(new_col) =
                    (0..c).rev().take_while(|x| grid[r][*x] == b'.').last()
                {
                    grid[r][c] = b'.';
                    grid[r][new_col] = b'O';
                    last = Some(new_col + 1);
                }
            } else {
                last = None;
            }
        }
    }
}

fn roll_south(grid: &mut Vec<Vec<u8>>) {
    let height = grid.len();

    for c in 0..grid[0].len() {
        let mut last: Option<usize> = None;

        for r in (0..height).rev() {
            if grid[r][c] == b'O' {
                if let Some(new_row) = last {
                    grid[r][c] = b'.';
                    grid[new_row][c] = b'O';
                    last = Some(new_row - 1);
                } else if let Some(new_row) =
                    (r + 1..height).take_while(|x| grid[*x][c] == b'.').last()
                {
                    grid[r][c] = b'.';
                    grid[new_row][c] = b'O';
                    last = Some(new_row - 1);
                }
            } else {
                last = None;
            }
        }
    }
}

fn roll_east(grid: &mut Vec<Vec<u8>>) {
    let width = grid[0].len();
    for gridr in grid.iter_mut() {
        let mut last: Option<usize> = None;

        for c in (0..width).rev() {
            if gridr[c] == b'O' {
                if let Some(new_col) = last {
                    gridr[c] = b'.';
                    gridr[new_col] = b'O';
                    last = Some(new_col - 1);
                } else if let Some(new_col) =
                    (c + 1..width).take_while(|x| gridr[*x] == b'.').last()
                {
                    gridr[c] = b'.';
                    gridr[new_col] = b'O';
                    last = Some(new_col - 1);
                }
            } else {
                last = None;
            }
        }
    }
}

fn cycle(grid: &mut Vec<Vec<u8>>) {
    roll_north(grid);
    roll_west(grid);
    roll_south(grid);
    roll_east(grid);
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

    roll_north(&mut dish);
    load(&dish)
}

#[aoc(day14, part2)]
pub fn part2(platform: &Dish) -> usize {
    let mut dish = platform.dish.to_vec();
    let mut seen = HashMap::new();

    let mut t = 0;
    while t < TARGET {
        t += 1;
        cycle(&mut dish);

        if let Some(old) = seen.get(&dish) {
            let cyc = t - old;
            let amt = (TARGET - t) / cyc;
            t += amt * cyc;
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
        let platform = generator(SAMPLE);
        println!("{:?}", platform);

        // push_left(&mut platform.dish[9]);

        // for i in 0..platform.dish.len() {
        //     push_left(&mut platform.dish[i]);
        // }
        // println!("{:?}", platform);

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
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
