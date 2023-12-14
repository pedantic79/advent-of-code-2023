use ahash::HashMapExt;
use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashMap as HashMap;

const TARGET: usize = 1_000_000_000;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Dish {
    grid: Vec<u8>,
    width: usize,
    height: usize,
}

impl std::fmt::Debug for Dish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.chunks(self.width) {
            writeln!(f, "{}", String::from_utf8_lossy(row))?;
        }

        Ok(())
    }
}

impl Dish {
    fn roll_north(&mut self) {
        let w = self.width;
        for c in 0..w {
            for r in 0..self.height {
                if self.grid[r * w + c] == b'O' {
                    if let Some(new_row) = (0..r)
                        .rev()
                        .take_while(|&x| self.grid[x * w + c] == b'.')
                        .last()
                    {
                        self.grid[r * w + c] = b'.';
                        self.grid[new_row * w + c] = b'O';
                    }
                }
            }
        }
    }

    fn roll_west(&mut self) {
        let w = self.width;
        for r in 0..self.height {
            for c in 0..w {
                if self.grid[r * w + c] == b'O' {
                    if let Some(new_col) = (0..c)
                        .rev()
                        .take_while(|&x| self.grid[r * w + x] == b'.')
                        .last()
                    {
                        self.grid[r * w + c] = b'.';
                        self.grid[r * w + new_col] = b'O';
                    }
                }
            }
        }
    }

    fn roll_south(&mut self) {
        let height = self.height;
        let w = self.width;
        for c in 0..w {
            for r in (0..height).rev() {
                if self.grid[r * w + c] == b'O' {
                    if let Some(new_row) = (r + 1..height)
                        .take_while(|&x| self.grid[x * w + c] == b'.')
                        .last()
                    {
                        self.grid[r * w + c] = b'.';
                        self.grid[new_row * w + c] = b'O';
                    }
                }
            }
        }
    }

    fn roll_east(&mut self) {
        let w = self.width;
        for r in 0..self.height {
            for c in (0..w).rev() {
                if self.grid[r * w + c] == b'O' {
                    if let Some(new_col) = (c + 1..w)
                        .take_while(|&x| self.grid[r * w + x] == b'.')
                        .last()
                    {
                        self.grid[r * w + c] = b'.';
                        self.grid[r * w + new_col] = b'O';
                    }
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.roll_north();
        self.roll_west();
        self.roll_south();
        self.roll_east();
    }

    fn load(&self) -> usize {
        let mut sum = 0;
        for row in 0..self.height {
            let mult = self.height - row;

            let row_offset = row * self.width;
            for col in 0..self.width {
                let col_offset = row_offset + col;
                if self.grid[col_offset] == b'O' {
                    sum += mult;
                }
            }
        }
        sum
    }
}

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Dish {
    let width = input.find('\n').unwrap();
    let height = input.len() / width;
    let mut grid = Vec::new();
    for line in input.lines() {
        grid.extend(line.bytes());
    }

    Dish {
        grid,
        width,
        height,
    }
}

#[aoc(day14, part1)]
pub fn part1(platform: &Dish) -> usize {
    let mut platform = platform.clone();

    platform.roll_north();
    platform.load()
}

#[aoc(day14, part2)]
pub fn part2(platform: &Dish) -> usize {
    let mut platform = platform.clone();
    let mut seen = HashMap::new();

    let mut t = 0;
    while t < TARGET {
        t += 1;
        platform.cycle();

        if let Some(old) = seen.get(&platform.grid) {
            let cyc = t - old;
            let amt = (TARGET - t) / cyc;
            t += amt * cyc;
        }

        seen.insert(platform.grid.clone(), t);
    }

    platform.load()
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
        let mut platform = generator(SAMPLE);
        println!("{:?}", platform);
        platform.roll_north();
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
