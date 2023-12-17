use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;

#[derive(Debug, PartialEq, Eq)]
pub struct Grid {
    grid: Vec<Vec<u8>>,
}

impl Grid {
    fn get_cost(&self, (y, x): (usize, usize)) -> Option<usize> {
        self.grid.get(y)?.get(x).copied().map(From::from)
    }

    #[allow(unused)]
    fn debug_grid(&self, ms: &[Movement]) {
        for (r, row) in self.grid.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if let Some(m) = ms.iter().find(|m| m.pos == (r, c)) {
                    let sym = match m.dir {
                        Direction::North => '^',
                        Direction::West => '<',
                        Direction::South => 'v',
                        Direction::East => '>',
                        Direction::None => '?',
                    };
                    print!("{sym}");
                } else {
                    print!("{cell}",);
                }
            }
            println!();
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East,
    None,
}

impl Direction {
    fn ninety_turn(&self) -> [Direction; 2] {
        match self {
            Direction::North | Direction::South => [Direction::East, Direction::West],
            Direction::West | Direction::East => [Direction::North, Direction::South],
            Direction::None => panic!("we should not call ninety_turn on a None"),
        }
    }

    fn next_pos(&self, (y, x): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (y.wrapping_sub(1), x),
            Direction::West => (y, x.wrapping_sub(1)),
            Direction::South => (y + 1, x),
            Direction::East => (y, x + 1),
            Direction::None => panic!("we should not call get_cost on a None"),
        }
    }

    fn reverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::None => Direction::None,
        }
    }
}

#[aoc_generator(day17)]
pub fn generator(input: &str) -> Grid {
    let grid = input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect();

    Grid { grid }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Movement {
    dir: Direction,
    repeat: usize,
    pos: (usize, usize),
}

#[aoc(day17, part1)]
pub fn part1(inputs: &Grid) -> usize {
    let height = inputs.grid.len();
    let width = inputs.grid[0].len();

    let ans = dijkstra(
        &Movement {
            dir: Direction::None,
            repeat: 0,
            pos: (0, 0),
        },
        |m| {
            if m.repeat == 3 {
                m.dir
                    .ninety_turn()
                    .into_iter()
                    .filter_map(|d| {
                        let new_pos = d.next_pos(m.pos);
                        let cost = inputs.get_cost(new_pos)?;

                        Some((
                            Movement {
                                dir: d,
                                repeat: 1,
                                pos: new_pos,
                            },
                            cost,
                        ))
                    })
                    .collect_vec()
            } else {
                [
                    Direction::North,
                    Direction::West,
                    Direction::South,
                    Direction::East,
                ]
                .into_iter()
                .filter_map(|d| {
                    if m.dir.reverse() == d {
                        return None;
                    }

                    let repeat = if m.dir == d { m.repeat + 1 } else { 1 };
                    let new_pos = d.next_pos(m.pos);
                    let cost = inputs.get_cost(new_pos)?;

                    Some((
                        Movement {
                            dir: d,
                            repeat,
                            pos: new_pos,
                        },
                        cost,
                    ))
                })
                .collect_vec()
            }
        },
        |m| m.pos == (height - 1, width - 1),
    )
    .unwrap();

    // inputs.debug_grid(&ans.0);
    // println!("{:?}", ans.0);

    ans.1
}

#[aoc(day17, part2)]
pub fn part2(inputs: &Grid) -> usize {
    let height = inputs.grid.len();
    let width = inputs.grid[0].len();

    let ans = dijkstra(
        &Movement {
            dir: Direction::None,
            repeat: 0,
            pos: (0, 0),
        },
        |m| {
            if m.repeat < 4 && m.dir != Direction::None {
                [m.dir]
                    .into_iter()
                    .filter_map(|d| {
                        let new_pos = d.next_pos(m.pos);
                        let cost = inputs.get_cost(new_pos)?;

                        Some((
                            Movement {
                                dir: d,
                                repeat: m.repeat + 1,
                                pos: new_pos,
                            },
                            cost,
                        ))
                    })
                    .collect_vec()
            } else if m.repeat == 10 && m.dir != Direction::None {
                m.dir
                    .ninety_turn()
                    .into_iter()
                    .filter_map(|d| {
                        let new_pos = d.next_pos(m.pos);
                        let cost = inputs.get_cost(new_pos)?;

                        Some((
                            Movement {
                                dir: d,
                                repeat: 1,
                                pos: new_pos,
                            },
                            cost,
                        ))
                    })
                    .collect_vec()
            } else {
                [
                    Direction::North,
                    Direction::West,
                    Direction::South,
                    Direction::East,
                ]
                .into_iter()
                .filter_map(|d| {
                    if m.dir.reverse() == d {
                        return None;
                    }

                    let repeat = if m.dir == d { m.repeat + 1 } else { 1 };
                    let new_pos = d.next_pos(m.pos);
                    let cost = inputs.get_cost(new_pos)?;

                    Some((
                        Movement {
                            dir: d,
                            repeat,
                            pos: new_pos,
                        },
                        cost,
                    ))
                })
                .collect_vec()
            }
        },
        |m| m.repeat > 3 && m.pos == (height - 1, width - 1),
    )
    .unwrap();

    // inputs.debug_grid(&ans.0);
    // println!("{:?}", ans.0);

    ans.1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 102);
    }

    #[test]
    pub fn part2_test() {
        const SAMPLE2: &str = "111111111111
999999999991
999999999991
999999999991
999999999991";
        assert_eq!(part2(&generator(SAMPLE)), 94);
        assert_eq!(part2(&generator(SAMPLE2)), 71);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day17.txt");
        const ANSWERS: (usize, usize) = (791, 900);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
