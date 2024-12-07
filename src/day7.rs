use core::panic;
use std::fs;

type Eqn = (u64, Vec<u64>);

fn parse_lines(input: &str) -> Vec<Eqn> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            if let Some((left, right)) = l.split_once(": ") {
                let this = (
                    left.parse::<u64>().unwrap(),
                    right
                        .split(' ')
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect(),
                );
                this
            } else {
                panic!();
            }
        })
        .collect()
}

fn part1(input: &str) -> u64 {
    parse_lines(input);
    0
}

const _TESTCASE: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20

";

pub fn run() {
    println!("Day 7");
    part1(&fs::read_to_string("data/day7.txt").unwrap());
    assert_eq!(part1(_TESTCASE), 3749);
}
