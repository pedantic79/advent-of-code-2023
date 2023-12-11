use aoc_runner_derive::aoc;

fn parse_data<const EXP: usize>(s: &str) -> Vec<(usize, usize)> {
    let width = s.lines().next().unwrap().len();
    let mut col_seen = vec![false; width];
    let mut row_seen = vec![];
    let mut grid = Vec::new();

    for (row, line) in s.lines().enumerate() {
        let mut seen = false;
        for (col, b) in line.bytes().enumerate() {
            if b == b'#' {
                col_seen[col] = true;
                seen = true;
                grid.push((row, col));
            }
        }
        row_seen.push(seen);
    }

    expand::<EXP, _>(&mut grid, &row_seen, |(galaxy, _)| galaxy);
    expand::<EXP, _>(&mut grid, &col_seen, |(_, galaxy)| galaxy);

    grid
}

fn expand<const EXP: usize, F>(grid: &mut [(usize, usize)], seen: &[bool], get: F)
where
    F: Fn(&mut (usize, usize)) -> &mut usize,
{
    let mut adjustment = 0;
    for (row, seen) in seen.iter().enumerate() {
        if !seen {
            for galaxy in grid.iter_mut() {
                if *get(galaxy) > row + adjustment {
                    *get(galaxy) += EXP;
                }
            }
            adjustment += EXP;
        }
    }
}

fn solve<const EXP: usize>(s: &str) -> usize {
    let v = parse_data::<EXP>(s);

    let mut sum = 0;
    for i in 0..v.len() {
        for j in i..v.len() {
            sum += v[i].0.abs_diff(v[j].0);
            sum += v[i].1.abs_diff(v[j].1);
        }
    }

    sum
}

#[aoc(day11, part1)]
pub fn part1(inputs: &str) -> usize {
    solve::<1>(inputs)
}

#[aoc(day11, part2)]
pub fn part2(inputs: &str) -> usize {
    solve::<999999>(inputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    pub fn input_test() {
        const RESULT: &str = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";

        let input: Vec<_> = RESULT
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.bytes()
                    .enumerate()
                    .filter(|(_, b)| b == &b'#')
                    .map(move |(col, _)| (row, col))
            })
            .collect();

        assert_eq!(parse_data::<1>(SAMPLE), input);
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(SAMPLE), 374);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(solve::<9>(SAMPLE), 1030);
        assert_eq!(solve::<99>(SAMPLE), 8410);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day11.txt");
        const ANSWERS: (usize, usize) = (10228230, 447073334102);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            // let output = generator(input);

            assert_eq!(part1(input), ANSWERS.0);
            assert_eq!(part2(input), ANSWERS.1);
        }
    }
}
