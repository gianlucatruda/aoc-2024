// https://adventofcode.com/2024/day/5

use std::fs;

fn parse_inp(input: &str) -> (Vec<(u16, u16)>, Vec<Vec<u16>>) {
    let (rstr, ustr) = input.split_once("\n\n").expect("Parsing input");

    (
        rstr.lines()
            .filter_map(|l| l.split_once("|"))
            .filter_map(|(l, r)| Some((l.parse::<u16>().unwrap(), r.parse::<u16>().unwrap())))
            .collect(),
        ustr.lines()
            .filter_map(|line| {
                if !line.is_empty() {
                    Some(line.split(',').map(|s| s.parse().unwrap()).collect())
                } else {
                    None
                }
            })
            .collect(),
    )
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    let (rules, updates) = parse_inp(input);
    for update in updates.iter() {
        let mut violated = false;
        for rule in rules.iter() {
            // Check if update violates rule
            if let Some(i_0) = update.iter().position(|&x| x == rule.0) {
                if let Some(i_1) = update.iter().position(|&x| x == rule.1) {
                    if i_0 > i_1 {
                        violated = true;
                        break;
                    }
                }
            }
        }
        if !violated {
            let mid = update[update.len() / 2];
            sum += mid;
        }
    }
    sum.into()
}

const _TESTCASE: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47

";

pub fn run() {
    assert_eq!(part1(_TESTCASE), 143);
    // assert_eq!(part2(_TESTCASE), 123);
    let input = fs::read_to_string("data/day5.txt").expect("Reading day5.txt");
    let a = part1(&input);
    println!("Day 5 part 1: {a}");
}
