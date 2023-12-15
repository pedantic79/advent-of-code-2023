use std::hash::BuildHasherDefault;

use aoc_runner_derive::aoc;

type IndexMap<K, V> = indexmap::IndexMap<K, V, BuildHasherDefault<rustc_hash::FxHasher>>;

#[derive(Debug, PartialEq, Eq)]
pub struct Object {}

fn calc_hash(s: &str) -> usize {
    s.bytes().fold(0, |mut current, n| {
        let n = usize::from(n);
        current += n;
        current *= 17;
        current %= 256;
        current
    })
}

// #[aoc_generator(day15)]
pub fn generator(s: &str) -> &str {
    s
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    input.split(',').map(calc_hash).sum()
}

#[aoc(day15, part2)]
pub fn part2(inputs: &str) -> usize {
    let mut boxes: [IndexMap<&str, usize>; 256] = std::array::from_fn(|_| IndexMap::default());
    for operation in inputs.split(',') {
        if let Some((label, focal_length)) = operation.split_once('=') {
            let focal_length = focal_length.parse::<usize>().unwrap();
            let label_num = calc_hash(label);

            if let Some(p) = boxes[label_num].get_mut(label) {
                *p = focal_length;
            } else {
                boxes[label_num].insert(label, focal_length);
            }
        } else if let Some(label) = operation.strip_suffix('-') {
            let label_num = calc_hash(label);
            boxes[label_num].shift_remove_entry(label);
        }
    }

    let mut sum = 0;
    for (box_num, v) in boxes.iter().enumerate() {
        if v.is_empty() {
            continue;
        }

        for (slot, (_, focal)) in v.iter().enumerate() {
            sum += (box_num + 1) * (slot + 1) * focal;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    pub fn input_test() {
        // println!("{:?}", generator(SAMPLE));

        println!("{:?}", calc_hash("H"));
        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(SAMPLE), 1320);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(SAMPLE), 145);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day15.txt");
        const ANSWERS: (usize, usize) = (501680, 241094);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
