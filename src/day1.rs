// https://adventofcode.com/2024/day/1

use std::{fs, iter::zip};

fn parse_and_sort_lists(input: String) -> (Vec<u32>, Vec<u32>) {
    let mut a: Vec<u32> = Vec::new();
    let mut b: Vec<u32> = Vec::new();

    for line in input.lines() {
        let data: Vec<&str> = line.split_whitespace().collect();
        if data.len() < 2 {
            continue;
        }
        let left: u32 = data[0].parse().expect("str to int");
        let right: u32 = data[1].parse().expect("str to int");

        a.push(left);
        b.push(right);
    }

    a.sort();
    b.sort();

    (a, b)
}

fn part1(a: &Vec<u32>, b: &Vec<u32>) -> u32 {
    let mut total: u32 = 0;
    for (l, r) in zip(a, b) {
        let val = match l >= r {
            true => l - r,
            _ => r - l,
        };
        total += val;
    }
    println!("Day 1 part 1: {total}");
    total
}

fn part2(a: &[u32], b: &[u32]) -> u32 {
    let mut score: u32 = 0;
    for av in a.iter() {
        let mut hits = 0;
        for bv in b.iter() {
            if av == bv {
                hits += 1;
            }
        }
        score += av * hits;
    }
    println!("Day 1 part 2: {score}");
    score
}

pub fn run() {
    let input = fs::read_to_string("data/day1.txt").expect("Read input");
    let (a, b) = parse_and_sort_lists(input);
    part1(&a, &b);
    part2(&a, &b);
}

const _TESTCASE: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

#[test]
fn p1() {
    let (a, b) = parse_and_sort_lists(_TESTCASE.to_string());
    assert_eq!(part1(&a, &b), 11);
}
#[test]
fn p2() {
    let (a, b) = parse_and_sort_lists(_TESTCASE.to_string());
    assert_eq!(part2(&a, &b), 31);
}
