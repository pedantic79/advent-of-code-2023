use aoc_runner_derive::aoc;

type Vec<T> = arrayvec::ArrayVec<T, 8>;

fn calc_hash(s: &str) -> usize {
    s.bytes()
        .fold(0, |current, n| (current + usize::from(n)) * 17 % 256)
}

// #[aoc_generator(day15)]
// this is here so the tests don't need to refer directly to part1 and part2 without the generator
pub fn generator(s: &str) -> &str {
    s
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    input.split(',').map(calc_hash).sum()
}

#[aoc(day15, part2)]
pub fn part2(inputs: &str) -> usize {
    let mut boxes: [_; 256] = std::array::from_fn(|_| Vec::new());
    for operation in inputs.split(',') {
        if let Some(label) = operation.strip_suffix('-') {
            let hash = calc_hash(label);
            if let Some(pos) = boxes[hash].iter().position(|(l, _)| *l == label) {
                boxes[hash].remove(pos);
            }
        } else if let Some((label, focal_length)) = operation.split_once('=') {
            let focal_length: usize = focal_length.parse().unwrap();
            let hash = calc_hash(label);

            if let Some(fl) = boxes[hash]
                .iter_mut()
                .find_map(|(l, fl)| (*l == label).then_some(fl))
            {
                *fl = focal_length;
            } else {
                boxes[hash].push((label, focal_length));
            }
        } else {
            panic!("unknown input `{operation}`");
        }
    }

    let mut sum = 0;
    for (box_num, v) in boxes.iter().enumerate() {
        for (slot_num, (_, focal)) in v.iter().enumerate() {
            sum += (box_num + 1) * (slot_num + 1) * focal;
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

            assert_eq!(part1(output), ANSWERS.0);
            assert_eq!(part2(output), ANSWERS.1);
        }
    }
}
