use aoc_runner_derive::aoc;
use rustworkx_core::{connectivity::stoer_wagner_min_cut, petgraph::prelude::UnGraphMap};

#[aoc(day25, part1)]
pub fn part1(input: &str) -> usize {
    let graph = UnGraphMap::<_, ()>::from_edges(input.lines().flat_map(|line| {
        let key = &line[..3];
        line[5..].split(' ').map(move |node| (key, node))
    }));

    let group = stoer_wagner_min_cut(&graph, |_| Ok::<_, ()>(1))
        .unwrap()
        .unwrap()
        .1;

    let g1 = group.len();
    let g2 = graph.node_count() - g1;

    g1 * g2
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    pub fn input_test() {
        // println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(SAMPLE), 54);
    }

    #[test]
    pub fn part2_test() {
        // assert_eq!(part2(&generator(SAMPLE)), 336);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day25.txt");
        const ANSWERS: (usize, usize) = (552695, 0);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            assert_eq!(part1(input), ANSWERS.0);
            // assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
