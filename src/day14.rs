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
    fn roll(
        &mut self,
        x: impl IntoIterator<Item = usize>,
        y: impl IntoIterator<Item = usize> + Clone,
        f: impl Fn(usize, usize) -> usize,
        offset: u8,
    ) {
        let mut available_slots = [u8::MAX; 100];
        for x in x {
            for y in y.clone() {
                let idx = f(x, y);
                let slot = &mut available_slots[y];

                match self.grid[idx] {
                    b'.' if *slot == u8::MAX => *slot = x as u8,
                    b'#' => *slot = u8::MAX,
                    b'O' if *slot != u8::MAX => {
                        self.grid[idx] = b'.';
                        self.grid[f(usize::from(*slot), y)] = b'O';
                        *slot = slot.wrapping_add(offset);
                    }
                    _ => {}
                }
            }
        }
    }

    fn roll_north(&mut self) {
        let w = self.width;
        self.roll(0..self.height, 0..w, |r, c| r * w + c, 1);
    }

    fn roll_west(&mut self) {
        let w = self.width;
        self.roll(0..w, 0..self.height, |c, r| r * w + c, 1);
    }

    fn roll_south(&mut self) {
        let w = self.width;
        let h = self.height;
        // 0u8.wrapping_sub(1) is -1
        self.roll((0..h).rev(), 0..w, |r, c| r * w + c, 0u8.wrapping_sub(1));
    }

    fn roll_east(&mut self) {
        let w = self.width;
        let h = self.height;
        self.roll((0..w).rev(), 0..h, |c, r| r * w + c, 0u8.wrapping_sub(1));
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
                if self.grid[row_offset + col] == b'O' {
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
    let mut grid = Vec::with_capacity(width * height);
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
    let mut seen = HashMap::with_capacity(256);

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
