use std::ops::Range;

use ahash::{HashMap, HashMapExt};
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1, take_while1},
    character::complete::{newline, one_of},
    combinator::{map, value},
    multi::separated_list0,
    IResult,
};

use crate::common::nom::{fold_separated_list0, nom_lines, nom_usize, process_input};

#[derive(Debug, PartialEq, Eq)]
pub struct Part([usize; 4]);

impl Part {
    fn get(&self, name: usize) -> usize {
        self.0[name]
    }

    fn check(&self, key: usize, cmp: char, n: usize) -> bool {
        if cmp == '<' {
            self.get(key) < n
        } else {
            self.get(key) > n
        }
    }

    fn rating(&self) -> usize {
        self.0.iter().sum()
    }
}

fn parse_part(i: &str) -> IResult<&str, Part> {
    let (i, _) = tag("{x=")(i)?;
    let (i, x) = nom_usize(i)?;
    let (i, _) = tag(",m=")(i)?;
    let (i, m) = nom_usize(i)?;
    let (i, _) = tag(",a=")(i)?;
    let (i, a) = nom_usize(i)?;
    let (i, _) = tag(",s=")(i)?;
    let (i, s) = nom_usize(i)?;
    let (i, _) = tag("}")(i)?;

    Ok((i, Part([x, m, a, s])))
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Parts([Range<usize>; 4]);

impl Default for Parts {
    fn default() -> Self {
        Self([1..4001, 1..4001, 1..4001, 1..4001])
    }
}

impl Parts {
    fn count(&self) -> usize {
        self.0.iter().map(|a| a.len()).product()
    }

    fn get(&self, key: usize) -> &Range<usize> {
        &self.0[key]
    }

    fn modify(&self, key: usize, r: Range<usize>) -> Self {
        let mut res = self.clone();
        res.0[key] = r;
        res
    }

    fn check(&self, key: usize, cmp: char, n: usize) -> [Option<Parts>; 2] {
        let r = self.get(key);
        let (t, f) = if cmp == '<' {
            ((r.start..n), (n..r.end))
        } else {
            // if x > n, then the first one that is included is n+1
            ((n + 1..r.end), (r.start..n + 1))
        };

        [
            (!t.is_empty()).then(|| self.modify(key, t)),
            (!f.is_empty()).then(|| self.modify(key, f)),
        ]
    }
}

#[derive(Debug, Clone)]
pub enum Jump {
    Rule(String),
    Reject,
    Accept,
}

fn parse_jump(s: &str) -> IResult<&str, Jump> {
    alt((
        value(Jump::Reject, tag("R")),
        value(Jump::Accept, tag("A")),
        map(take_while1(|c: char| c.is_ascii_alphabetic()), |s: &str| {
            Jump::Rule(s.into())
        }),
    ))(s)
}

#[derive(Debug)]
pub struct Workflow {
    rules: Vec<(usize, char, usize, Jump)>,
    default: Jump,
}

fn parse_rule(s: &str) -> IResult<&str, (usize, char, usize, Jump)> {
    let (s, xmas) = map(one_of("xmas"), |k: char| match k {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        _ => unreachable!("unknown xmas"),
    })(s)?;
    let (s, op) = one_of("<>")(s)?;
    let (s, n) = nom_usize(s)?;
    let (s, _) = tag(":")(s)?;
    let (s, jump) = parse_jump(s)?;

    Ok((s, (xmas, op, n, jump)))
}

fn parse_workflow(s: &str) -> IResult<&str, (String, Workflow)> {
    let (s, name) = take_until1("{")(s)?;
    let (s, _) = tag("{")(s)?;
    let (s, rules) = separated_list0(tag(","), parse_rule)(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, default) = parse_jump(s)?;
    let (s, _) = tag("}")(s)?;

    Ok((s, (name.to_string(), Workflow { rules, default })))
}

#[aoc_generator(day19)]
pub fn generator(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let (a, b) = input.split_once("\n\n").unwrap();

    let hm = process_input(fold_separated_list0(
        newline,
        parse_workflow,
        HashMap::new,
        |mut hm, (name, wf)| {
            hm.insert(name, wf);
            hm
        },
    ))(a);

    let parts = process_input(nom_lines(parse_part))(b);

    (hm, parts)
}

fn check_part(name: &Jump, p: &Part, wfs: &HashMap<String, Workflow>) -> bool {
    match name {
        Jump::Accept => true,
        Jump::Reject => false,
        Jump::Rule(name) => {
            if let Some(wf) = wfs.get(name) {
                let tgt = wf
                    .rules
                    .iter()
                    .find(|(key, cmp, n, _)| p.check(*key, *cmp, *n))
                    .map(|(_, _, _, tgt)| tgt)
                    .unwrap_or(&wf.default);

                check_part(tgt, p, wfs)
            } else {
                unreachable!("unknown name of workflow")
            }
        }
    }
}

fn count_parts(name: &Jump, ps: Parts, wfs: &HashMap<String, Workflow>) -> usize {
    match name {
        Jump::Accept => ps.count(),
        Jump::Reject => 0,
        Jump::Rule(name) => {
            let mut count = 0;
            if let Some(wf) = wfs.get(name) {
                let fin = wf.rules.iter().try_fold(ps, |ps, (key, cmp, n, tgt)| {
                    let [t, f] = ps.check(*key, *cmp, *n);
                    if let Some(t) = t {
                        count += count_parts(tgt, t, wfs);
                    }
                    f
                });

                if let Some(f) = fin {
                    count += count_parts(&wf.default, f, wfs);
                }

                count
            } else {
                unreachable!("unknown name of workflow")
            }
        }
    }
}

#[aoc(day19, part1)]
pub fn part1((wfs, parts): &(HashMap<String, Workflow>, Vec<Part>)) -> usize {
    let start = Jump::Rule("in".into());
    parts
        .iter()
        .filter(|p| check_part(&start, p, wfs))
        .map(|p| p.rating())
        .sum()
}

#[aoc(day19, part2)]
pub fn part2((wfs, _): &(HashMap<String, Workflow>, Vec<Part>)) -> usize {
    count_parts(&Jump::Rule("in".into()), Parts::default(), wfs)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 19114);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 167409079868000);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2023/day19.txt");
        const ANSWERS: (usize, usize) = (406934, 131192538505367);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
