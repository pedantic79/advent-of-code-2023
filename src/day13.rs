use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Pattern {
    pattern: Vec<Vec<u8>>,
}

fn parse(s: &str) -> Pattern {
    let pattern: Vec<_> = s.lines().map(|line| line.as_bytes().to_vec()).collect();
    Pattern { pattern }
}

fn rotate(matrix: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut rotated_matrix: Vec<Vec<u8>> = vec![vec![0; rows]; cols];

    for i in 0..rows {
        for (j, &cell) in matrix[rows - 1 - i].iter().enumerate() {
            rotated_matrix[j][i] = cell;
        }
    }

    rotated_matrix
}

// fn check_mirror(left: &[Vec<u8>], right: &[Vec<u8>]) -> bool {
//     left.iter().rev().zip(right.iter()).all(|(l, r)| l == r)
// }

fn count_badness(left: &[u8], right: &[u8]) -> usize {
    left.iter()
        .zip(right.iter())
        .filter(|(l, r)| l != r)
        .count()
}

fn check_mirror(left: &[Vec<u8>], right: &[Vec<u8>]) -> usize {
    left.iter()
        .rev()
        .zip(right.iter())
        .map(|(l, r)| count_badness(l, r))
        .sum()
}

fn find_mirror_horz<const TARGET: usize>(p: &[Vec<u8>]) -> Option<usize> {
    (1..p.len()).find(|&split| {
        let (left, right) = p.split_at(split);
        check_mirror(left, right) == TARGET
    })
}

fn find_mirror_vert<const TARGET: usize>(p: &[Vec<u8>]) -> Option<usize> {
    find_mirror_horz::<TARGET>(&rotate(p))
}

fn find_mirror<const TARGET: usize>(p: &Pattern) -> Option<usize> {
    let a = find_mirror_horz::<TARGET>(&p.pattern);

    if let Some(a) = a {
        Some(a * 100)
    } else {
        find_mirror_vert::<TARGET>(&p.pattern)
    }
}

#[aoc_generator(day13)]
pub fn generator(input: &str) -> Vec<Pattern> {
    input.split("\n\n").map(parse).collect()
}

#[aoc(day13, part1)]
pub fn part1(inputs: &[Pattern]) -> usize {
    inputs.iter().filter_map(find_mirror::<0>).sum()
}

#[aoc(day13, part2)]
pub fn part2(inputs: &[Pattern]) -> usize {
    inputs.iter().filter_map(find_mirror::<1>).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    pub fn input_test() {
        let p = generator(SAMPLE);
        // println!("{:?}", p);

        assert_eq!(find_mirror_vert::<0>(&p[0].pattern), Some(5));
        assert_eq!(find_mirror_horz::<0>(&p[1].pattern), Some(4));
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 405);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 400);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day13.txt");
        const ANSWERS: (usize, usize) = (33047, 28806);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
