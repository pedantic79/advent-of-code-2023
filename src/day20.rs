use std::collections::VecDeque;

use ahash::{HashMap, HashMapExt};
use aoc_runner_derive::{aoc, aoc_generator};
use arrayvec::ArrayVec;
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{newline, one_of},
    multi::separated_list0,
    IResult,
};
use num::Integer;
type String = smallstr::SmallString<[u8; 2]>;

use crate::common::nom::{fold_separated_list0, process_input};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Signal {
    Low,
    High,
}

impl std::ops::Not for Signal {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Signal::Low => Signal::High,
            Signal::High => Signal::Low,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Module {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<String, Signal>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Machine {
    m_type: Module,
    id: String,
    output: ArrayVec<String, 8>,
}

impl Machine {
    fn pulse(
        &mut self,
        sender: &String,
        signal: Signal,
        queue: &mut impl Extend<(String, String, Signal)>,
    ) {
        match &mut self.m_type {
            Module::Broadcaster => self.generate_output(signal, queue),
            Module::FlipFlop(status) => {
                if signal == Signal::Low {
                    let new_status = !*status;
                    *status = new_status;

                    self.generate_output(
                        if new_status {
                            Signal::High
                        } else {
                            Signal::Low
                        },
                        queue,
                    )
                }
            }
            Module::Conjunction(memory) => {
                *memory.get_mut(sender).unwrap() = signal;
                let new_signal = if memory.values().all(|s| s == &Signal::High) {
                    Signal::Low
                } else {
                    Signal::High
                };

                self.generate_output(new_signal, queue)
            }
        }
    }

    fn generate_output(&self, signal: Signal, queue: &mut impl Extend<(String, String, Signal)>) {
        queue.extend(
            self.output
                .iter()
                .map(|id| (self.id.clone(), id.clone(), signal)),
        )
    }
}

fn parse_connection(s: &str) -> IResult<&str, (String, ArrayVec<String, 8>)> {
    let (s, name) = take_while1(|c: char| c.is_alphabetic())(s)?;
    let (s, _) = tag(" -> ")(s)?;
    let (s, res) = fold_separated_list0(
        tag(", "),
        take_while1(|c: char| c.is_alphabetic()),
        ArrayVec::new,
        |mut v, x: &str| {
            v.push(x.into());
            v
        },
    )(s)?;

    Ok((s, (name.into(), res)))
}

fn parse_module(s: &str) -> IResult<&str, Machine> {
    let (s, typ) = one_of("%&b")(s)?;
    let (s, (key, output)) = parse_connection(s)?;
    let res = match typ {
        'b' => Machine {
            m_type: Module::Broadcaster,
            id: "br".into(),
            output,
        },
        '%' => Machine {
            m_type: Module::FlipFlop(false),
            id: key.to_owned(),
            output,
        },
        '&' => Machine {
            m_type: Module::Conjunction(HashMap::new()),
            id: key.to_owned(),
            output,
        },
        _ => panic!("foo"),
    };

    Ok((s, res))
}

#[aoc_generator(day20)]
pub fn generator(input: &str) -> Vec<Machine> {
    let mut machines = process_input(separated_list0(newline, parse_module))(input);

    // Find all conjuctions, and the HashMap
    let mut conjunctions: HashMap<String, HashMap<String, Signal>> = machines
        .iter()
        .filter_map(|m| {
            if let Module::Conjunction(_) = m.m_type {
                Some((m.id.clone(), HashMap::new()))
            } else {
                None
            }
        })
        .collect();

    // Run though each machine, if the outputs are contained in the conjunctions hashmap, then add the id
    for m in machines.iter() {
        for out in m.output.iter() {
            if let Some(hm) = conjunctions.get_mut(out) {
                hm.insert(m.id.clone(), Signal::Low);
            }
        }
    }

    // Run through the machines again, and assign the HashMap in conjunctions to the conjunction memory
    for m in machines.iter_mut() {
        if let Module::Conjunction(memory) = &mut m.m_type {
            let id = m.id.clone();
            if let Some(hm) = conjunctions.remove(&id) {
                *memory = hm;
            }
        }
    }

    machines
}

#[aoc(day20, part1)]
pub fn part1(inputs: &[Machine]) -> u64 {
    let mut machines = inputs.iter().fold(HashMap::new(), |mut hm, m| {
        hm.insert(m.id.clone(), m.clone());
        hm
    });
    let mut queue = VecDeque::with_capacity(64);
    let mut high = 0;
    let mut low = 0;

    for _ in 0..1000 {
        assert!(queue.is_empty());
        queue.push_back((String::from("bu"), String::from("br"), Signal::Low));

        while let Some((fr, to, signal)) = queue.pop_front() {
            match signal {
                Signal::High => {
                    high += 1;
                }
                Signal::Low => {
                    low += 1;
                }
            }

            if let Some(m) = machines.get_mut(&to) {
                m.pulse(&fr, signal, &mut queue);
            }
        }
    }

    high * low
}

#[aoc(day20, part2)]
pub fn part2(inputs: &[Machine]) -> u64 {
    let mut machines = inputs.iter().fold(HashMap::new(), |mut hm, m| {
        hm.insert(m.id.clone(), m.clone());
        hm
    });

    let key_conjunction = inputs
        .iter()
        .find(|m| m.output.contains(&String::from("rx")))
        .unwrap();

    let mut penultimate: Vec<String> = inputs
        .iter()
        .filter(|m| m.output.contains(&key_conjunction.id))
        .map(|m| m.id.clone())
        .collect();

    let mut frequency = Vec::with_capacity(penultimate.len());

    let mut queue = VecDeque::with_capacity(64);
    for i in 1.. {
        assert!(queue.is_empty());
        queue.push_back((String::from("bu"), String::from("br"), Signal::Low));

        while let Some((fr, to, signal)) = queue.pop_front() {
            if let Some(pos) = penultimate.iter().position(|k| k == &to) {
                if signal == Signal::Low {
                    frequency.push(i);

                    penultimate.remove(pos);
                    if penultimate.is_empty() {
                        // println!("{frequency:?}");
                        return frequency.iter().fold(1, |lcm, v| lcm.lcm(v));
                    }
                }
            }

            if let Some(m) = machines.get_mut(&to) {
                m.pulse(&fr, signal, &mut queue);
            }
        }
    }

    panic!("shouldn't reach here")
    // println!("{:?}", frequency);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 32000000);
    }

    #[test]
    pub fn part2_test() {
        // assert_eq!(part2(&generator(SAMPLE)), 336);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day20.txt");
        const ANSWERS: (u64, u64) = (788848550, 228300182686739);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
