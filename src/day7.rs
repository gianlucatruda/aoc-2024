use core::panic;
use itertools::{repeat_n, Itertools};
use std::fs;

type Eqn = (u64, Vec<u64>);

#[derive(Debug, Clone)]
enum Op {
    Plus,
    Times,
    Cat,
}

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
                panic!("Summit ain't right 'arry");
            }
        })
        .collect()
}

fn cat(a: u64, b: u64) -> u64 {
    {a.to_string() + &b.to_string()}.parse().unwrap()

}

fn calc(parts: &[u64], perm: &[Op]) -> u64 {
    let mut total = parts[0];
    let mut i = 0;
    while i < perm.len() {
        match perm[i] {
            Op::Plus => total += parts[i + 1],
            Op::Times => total *= parts[i + 1],
            Op::Cat => total = cat(total, parts[i + 1]),
        }
        i += 1;
    }
    total
}

fn valid(eq: &Eqn, ops: Vec<Op>) -> bool {
    // println!("{eq:?}");
    let (t, parts) = eq;
    let perms = repeat_n(ops.into_iter(), parts.len() - 1).multi_cartesian_product();
    for perm in perms {
        // println!("{perm:?}");
        if calc(parts, &perm) == *t {
            // println!("Solved: {eq:?} with {perm:?}");
            return true;
        }
    }

    false
}

fn part1(input: &str) -> u64 {
    let equations = parse_lines(input);
    let mut sum = 0;
    for eq in equations.iter() {
        if valid(eq, vec![Op::Plus, Op::Times]) {
            sum += eq.0;
        }
    }

    sum
}

fn part2(input: &str) -> u64 {
    let equations = parse_lines(input);
    let mut sum = 0;
    for eq in equations.iter() {
        if valid(eq, vec![Op::Plus, Op::Times, Op::Cat]) {
            sum += eq.0;
        }
    }

    sum
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
    // assert_eq!(cat(123, 4567), 1234567);
    // assert_eq!(cat(1, 234567), 1234567);
    // assert_eq!(part2(_TESTCASE), 11387);
    let input = fs::read_to_string("data/day7.txt").expect("Reading day7.txt");
    let a = part1(&input);
    println!("Day 7 part 1: {a}");
    let b = part2(&input);
    println!("Day 7 part 2: {b}");
}

#[test]
fn p1() {
    assert_eq!(part1(_TESTCASE), 3749);
}
#[test]
fn p2() {
    assert_eq!(part2(_TESTCASE), 11387);
}
