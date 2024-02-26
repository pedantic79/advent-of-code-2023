use ahash::HashMapExt;
use aoc_runner_derive::{aoc, aoc_generator};
use arrayvec::ArrayVec;
use pathfinding::{
    directed::bfs,
    matrix::{
        directions::{self, DIRECTIONS_4},
        Matrix,
    },
};
use petgraph::{algo::all_simple_paths, Graph};
use rustc_hash::FxHashMap as HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Object {}

#[aoc_generator(day23)]
pub fn generator(input: &str) -> Matrix<u8> {
    input.lines().map(|l| l.bytes()).collect()
}

fn get_direction(c: u8) -> (isize, isize) {
    match c {
        b'>' => directions::E,
        b'<' => directions::W,
        b'^' => directions::N,
        b'v' => directions::S,
        _ => panic!("unknown direction"),
    }
}

fn solve<F, T>(grid: &Matrix<u8>, successors: F, mut g: Graph<(usize, usize), usize, T>) -> usize
where
    F: Fn((usize, usize), &Matrix<u8>) -> ArrayVec<(usize, usize), 4>,
    T: petgraph::EdgeType,
{
    let start = (0, 1);
    let end = (grid.rows - 1, grid.columns - 2);

    let check_valid = |p| grid.get(p) != Some(&b'#');

    // find point of interests
    // let mut g = Graph::new();
    let mut h = HashMap::with_capacity(36);
    let mut nodes = Vec::with_capacity(36);
    for row in 0..grid.rows {
        for col in 0..grid.columns {
            let p = (row, col);
            if check_valid(p) && (successors(p, grid).len() > 2 || p == start || p == end) {
                h.insert(p, g.add_node(p));
                nodes.push(p);
            }
        }
    }

    // shrink
    for &n in nodes.iter() {
        let mut seen = vec![false; grid.len()];
        let l = 1.max(successors(n, grid).len());
        for _ in 0..l {
            let path = bfs::bfs(
                &n,
                |&neighbors| {
                    if seen[grid.idx(neighbors)]
                        || (h.contains_key(&neighbors) && g.contains_edge(h[&neighbors], h[&n]))
                    {
                        ArrayVec::new()
                    } else {
                        successors(neighbors, grid)
                    }
                },
                |neighbors| {
                    *neighbors != n
                        && h.contains_key(neighbors)
                        && !seen[grid.idx(*neighbors)]
                        && !g.contains_edge(h[&neighbors], h[&n])
                },
            );
            if let Some(path) = path {
                let neighbors = path.last().copied().unwrap();
                seen[grid.idx(neighbors)] = true;

                g.add_edge(h[&n], h[&neighbors], path.len() - 1);
            }
        }
    }

    all_simple_paths::<ArrayVec<_, 36>, _>(&g, h[&start], h[&end], 0, None)
        .map(|path| {
            path.windows(2)
                .map(|w| g.edges_connecting(w[0], w[1]).next().unwrap().weight())
                .sum::<usize>()
        })
        .max()
        .unwrap()
}

#[aoc(day23, part1)]
pub fn part1(inputs: &Matrix<u8>) -> usize {
    solve(
        inputs, // find all neighbors
        |p, grid| {
            let check_valid = |p| grid.get(p) != Some(&b'#');

            let mut neighbors = ArrayVec::<_, 4>::new();
            match grid.get(p) {
                Some(b'#') => (),
                Some(&d @ (b'>' | b'<' | b'^' | b'v')) => {
                    if let Some(next_p) = grid.move_in_direction(p, get_direction(d)) {
                        if check_valid(next_p) {
                            neighbors.push(next_p);
                        }
                    }
                }
                Some(b'.') => {
                    neighbors.extend(DIRECTIONS_4.iter().filter_map(|d| {
                        grid.move_in_direction(p, *d)
                            .filter(|&next_p| check_valid(next_p))
                    }));
                }
                _ => panic!("unexpected grid value"),
            }
            neighbors
        },
        Graph::new(),
    )
}

#[aoc(day23, part2)]
pub fn part2(inputs: &Matrix<u8>) -> usize {
    solve(
        inputs, // find all neighbors
        |p, grid| {
            let check_valid = |p| grid.get(p) != Some(&b'#');

            let mut neighbors = ArrayVec::<_, 4>::new();
            match grid.get(p) {
                Some(b'#') => (),
                Some(_) => {
                    neighbors.extend(DIRECTIONS_4.iter().filter_map(|d| {
                        grid.move_in_direction(p, *d)
                            .filter(|&next_p| check_valid(next_p))
                    }));
                }
                _ => panic!("unexpected grid value"),
            }
            neighbors
        },
        Graph::new_undirected(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 94);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 154);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day23.txt");
        const ANSWERS: (usize, usize) = (2010, 6318);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            // assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
