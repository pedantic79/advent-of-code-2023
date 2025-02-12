use aoc_runner_derive::{aoc, aoc_generator};
use nom::{character::complete::char, sequence::separated_pair, IResult, Parser};

use crate::common::nom::{nom_lines, nom_usize, process_input};

#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    x: (usize, usize),
    y: (usize, usize),
    z: (usize, usize),
}

fn parse_block(s: &str) -> IResult<&str, Block> {
    let (s, ((x0, _, y0, _, z0), (x1, _, y1, _, z1))) = separated_pair(
        (nom_usize, char(','), nom_usize, char(','), nom_usize),
        char('~'),
        (nom_usize, char(','), nom_usize, char(','), nom_usize),
    )
    .parse(s)?;

    Ok((
        s,
        Block {
            x: (x0, x1),
            y: (y0, y1),
            z: (z0, z1),
        },
    ))
}

#[aoc_generator(day22)]
pub fn generator(input: &str) -> Vec<Block> {
    let mut blocks = process_input(nom_lines(parse_block))(input);
    blocks.sort_by_key(|b| b.z.0);
    blocks
}

const DIM: usize = 10;

fn solve(blocks: &[Block]) -> (Vec<bool>, Vec<(usize, usize)>) {
    let mut heights = [0; DIM * DIM];
    let mut indices = [usize::MAX; DIM * DIM];

    let mut safe = vec![true; blocks.len()];
    let mut dominator = Vec::with_capacity(blocks.len());

    // for each block, loop through
    for (i, &Block { x, y, z }) in blocks.iter().enumerate() {
        // find the position in the heights/indices map
        let start = DIM * y.0 + x.0;
        let end = DIM * y.1 + x.1 + 1;
        let step = if y.1 > y.0 { DIM } else { 1 };

        let blk_hgt = z.1 - z.0 + 1;

        let mut previous = usize::MAX;
        let mut underneath = 0;
        let mut parent = 0;
        let mut depth = 0;

        // find the tallest height of all the pieces
        let top = (start..end)
            .step_by(step)
            .map(|j| heights[j])
            .max()
            .unwrap();

        for j in (start..end).step_by(step) {
            if heights[j] == top {
                let index = indices[j];

                if index != previous {
                    previous = index;
                    underneath += 1;

                    (parent, depth) = if underneath == 1 {
                        dominator[previous]
                    } else {
                        // Find common ancestor
                        let (mut a, mut d1) = (parent, depth);
                        let (mut x, mut d2) = dominator[previous];

                        while d1 > d2 {
                            (a, d1) = dominator[a];
                        }
                        while d2 > d1 {
                            (x, d2) = dominator[x];
                        }
                        while a != x {
                            (a, d1) = dominator[a];
                            (x, _) = dominator[x];
                        }

                        (a, d1)
                    }
                }
            }

            heights[j] = top + blk_hgt;
            indices[j] = i;
        }

        if underneath == 1 {
            safe[previous] = false;
            parent = previous;
            depth = dominator[previous].1 + 1;
        }

        dominator.push((parent, depth));
    }

    (safe, dominator)
}

#[aoc(day22, part1)]
pub fn part1(inputs: &[Block]) -> usize {
    solve(inputs).0.iter().filter(|&&b| b).count()
}

#[aoc(day22, part2)]
pub fn part2(inputs: &[Block]) -> usize {
    solve(inputs).1.iter().map(|&(_, d)| d).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 5);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 7);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day22.txt");
        const ANSWERS: (usize, usize) = (499, 95059);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
