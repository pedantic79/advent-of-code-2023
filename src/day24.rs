use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nalgebra::{Matrix6, RowVector6, Vector6};
use nom::{
    bytes::complete::tag, character::complete::space1, combinator::map, sequence::separated_pair,
    IResult, Parser,
};

use crate::common::nom::{nom_i64, nom_lines, process_input};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Hailstone {
    pos: Point,
    vel: Point,
}

impl Hailstone {
    fn solve_linear(
        (a1, b1, c1): (f64, f64, f64),
        (a2, b2, c2): (f64, f64, f64),
    ) -> Option<(f64, f64)> {
        let det = a1 * b2 - a2 * b1;
        if det == 0.0 {
            None
        } else {
            let x = (b2 * c1 - b1 * c2) / det;
            let y = (a1 * c2 - a2 * c1) / det;
            Some((x, y))
        }
    }

    fn intersection_xy(&self, other: &Self) -> Option<Point> {
        let (x0, y0) = (self.pos.x as f64, self.pos.y as f64);
        let (x1, y1) = (other.pos.x as f64, other.pos.y as f64);
        let (vx0, vy0) = (self.vel.x as f64, self.vel.y as f64);
        let (vx1, vy1) = (other.vel.x as f64, other.vel.y as f64);

        let c0 = x1 - x0;
        let c1 = y1 - y0;
        let (a0, b0) = (vx0, -vx1);
        let (a1, b1) = (vy0, -vy1);

        if let Some((s, t)) = Self::solve_linear((a0, b0, c0), (a1, b1, c1)) {
            if s >= 0.0 && t >= 0.0 {
                let xs = x0 + vx0 * s;
                let ys = y0 + vy0 * s;
                let xt = x1 + vx1 * t;
                let yt = y1 + vy1 * t;

                let x = (xs + xt) / 2.0;
                let y = (ys + yt) / 2.0;

                Some(Point {
                    x: x as i64,
                    y: y as i64,
                    z: 0,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn parse_symbol<'a>(beginning: &'a str) -> impl Fn(&'a str) -> IResult<&'a str, ()> {
    move |s: &str| {
        let (s, _) = tag(beginning)(s)?;
        let (s, _) = space1(s)?;
        Ok((s, ()))
    }
}

fn parse_point(s: &str) -> IResult<&str, Point> {
    map(
        (
            nom_i64,
            parse_symbol(","),
            nom_i64,
            parse_symbol(","),
            nom_i64,
        ),
        |(x, _, y, _, z)| Point { x, y, z },
    )
    .parse(s)
}

fn parse_hailstone(s: &str) -> IResult<&str, Hailstone> {
    map(
        separated_pair(parse_point, parse_symbol(" @"), parse_point),
        |(pos, vel)| Hailstone { pos, vel },
    )
    .parse(s)
}

#[aoc_generator(day24)]
pub fn generator(input: &str) -> Vec<Hailstone> {
    process_input(nom_lines(parse_hailstone))(input)
}

fn solve_part1<const MIN: i64, const MAX: i64>(hailstones: &[Hailstone]) -> usize {
    let intersections = hailstones
        .iter()
        .combinations(2)
        .filter_map(|hstones| Hailstone::intersection_xy(hstones[0], hstones[1]))
        .filter(|point| (MIN..=MAX).contains(&point.x) && (MIN..=MAX).contains(&point.y))
        .collect::<Vec<_>>();

    intersections.len()
}

#[aoc(day24, part1)]
pub fn part1(hailstones: &[Hailstone]) -> usize {
    solve_part1::<200000000000000, 400000000000000>(hailstones)
}

#[aoc(day24, part2)]
pub fn part2(hailstones: &[Hailstone]) -> u64 {
    // For part2, we are trying to find a rock PX, PY, PZ and VX, VY, VZ
    // This rock needs to hit another rock p0x, p0y, p0z, v0x, v0y, v0z
    //
    // For the x-coordinate, P and p0 intersect at time t
    // PX + t * VX = p0x + t * v0x
    // t = (PX - p0x)/(v0x - VX)
    //
    // Repeat for the y-coordinate:
    // t = (PY - p0y)/(v0y - VY)
    //
    // Now set each to be equal:
    // (PX - p0x)/(v0x - VX) = (PY - p0y)/(v0y - VY)
    //
    // Cross multiply to remove the division, and then multiply the bionomials
    // (PX - p0x)*(v0y - VY) = (PY - p0y)*(v0x - VX)
    // PX*v0y - PX*VY - p0x*v0y + p0x*VY = PY*v0x - PY*VX - p0y*v0x + p0y*VX
    //
    // We group only the terms dealing with p0 on the right
    // PY*VX - PX*VY = p0x*v0y - p0y*v0x + PY*v0x + p0y*VX - p0x*VY - PX*v0y
    //
    // Now repeat with another rock p1x, p1y, p1z, v1x, v1y, v1z
    // PY*VX - PX*VY = p1x*v1y - p1y*v1x + PY*v1x + p1y*VX - p1x*VY - PX*v1y
    //
    // As the left side of both are equal, we can set these two equations equal to each other
    // p0x*v0y - p0y*v0x + PY*v0x + p0y*VX - p0x*VY - PX*v0y = p1x*v1y - p1y*v1x + PY*v1x + p1y*VX - p1x*VY - PX*v1y
    //
    // Gather like terms with the unknown rock on the left, and the only known terms on the right
    // PX*(v1y - v0y) + PY*(v0x - v1x) + VX*(p0y - p1y) + VY*(v0x - v1x) = p0y*v0x - p0x*v0y + p1x*v1y - p1y*v1x
    //
    // This yeilds us one line in the 6x6 matrix A and one entry of B vector
    // [v1y - v0y, v0x - v1x, 0, p0y - p1y, v0x - v1x, 0] = [p0y*v0x - p0x*v0y + p1x*v1y - p1y*v1x]
    //
    // Repeat with xz and yz for rock p0 and rock p1
    // This will yield us an additional two lines to the matrix and vector
    //
    // Repeat the entire process with p0 and another rock p2
    // this yields us 6 equations for the 6 unknowns PX, PY, PZ, VX, VY, VYZ

    // Only use the first 3 hailstones
    let p0 = &hailstones[0].pos;
    let p1 = &hailstones[1].pos;
    let p2 = &hailstones[2].pos;
    let v0 = &hailstones[0].vel;
    let v1 = &hailstones[1].vel;
    let v2 = &hailstones[2].vel;

    let mk_rv6 = |v0: i64, v1: i64, v2: i64, p0: i64, p1: i64, p2: i64| {
        RowVector6::new(
            v0 as f64, v1 as f64, v2 as f64, p0 as f64, p1 as f64, p2 as f64,
        )
    };

    // coefficient matrix
    let a = Matrix6::from_rows(&[
        mk_rv6(v1.y - v0.y, v0.x - v1.x, 0, p0.y - p1.y, p1.x - p0.x, 0),
        mk_rv6(v2.y - v0.y, v0.x - v2.x, 0, p0.y - p2.y, p2.x - p0.x, 0),
        mk_rv6(v1.z - v0.z, 0, v0.x - v1.x, p0.z - p1.z, 0, p1.x - p0.x),
        mk_rv6(v2.z - v0.z, 0, v0.x - v2.x, p0.z - p2.z, 0, p2.x - p0.x),
        mk_rv6(0, v1.z - v0.z, v0.y - v1.y, 0, p0.z - p1.z, p1.y - p0.y),
        mk_rv6(0, v2.z - v0.z, v0.y - v2.y, 0, p0.z - p2.z, p2.y - p0.y),
    ]);

    // constraits vector
    let b = Vector6::from_row_slice(&[
        ((p0.y * v0.x - p1.y * v1.x) - (p0.x * v0.y - p1.x * v1.y)) as f64,
        ((p0.y * v0.x - p2.y * v2.x) - (p0.x * v0.y - p2.x * v2.y)) as f64,
        ((p0.z * v0.x - p1.z * v1.x) - (p0.x * v0.z - p1.x * v1.z)) as f64,
        ((p0.z * v0.x - p2.z * v2.x) - (p0.x * v0.z - p2.x * v2.z)) as f64,
        ((p0.z * v0.y - p1.z * v1.y) - (p0.y * v0.z - p1.y * v1.z)) as f64,
        ((p0.z * v0.y - p2.z * v2.y) - (p0.y * v0.z - p2.y * v2.z)) as f64,
    ]);

    let r = a.lu().solve(&b).unwrap();
    assert_eq!(r.len(), 6);

    r.iter().take(3).sum::<f64>().round() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(solve_part1::<7, 27>(&generator(SAMPLE)), 2);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 47);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day24.txt");
        const ANSWERS: (usize, u64) = (21679, 566914635762564);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
