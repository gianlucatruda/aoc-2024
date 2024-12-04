// https://adventofcode.com/2024/day/2

use std::fs;

fn parse_to_matrix(input: String) -> Vec<Vec<i32>> {
    let mut data: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        // Account for empty lines (e.g. at EoF)
        if line.is_empty() {
            continue;
        }
        let vals: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        data.push(vals);
    }

    data
}

fn is_safe(report: Vec<i32>) -> bool {
    let dir = report[1] - report[0];
    for (i, l) in report.iter().enumerate() {
        if i == report.len() - 1 {
            return true;
        }
        let d = report[i + 1] - l;
        // Changes direction
        if d * dir <= 0 {
            break;
        }
        // Out out bounds
        if d.abs() > 3 || d.abs() < 1 {
            break;
        }
    }

    false
}

fn part1(data: &[Vec<i32>]) -> u32 {
    let sum = data
        .iter()
        .fold(0, |a, x| if is_safe(x.to_vec()) { a + 1 } else { a });
    println!("Day 2 part 1: {sum}");
    sum
}

fn part2(data: &[Vec<i32>]) -> u32 {
    let mut sum = 0;
    for report in data.iter() {
        if is_safe(report.to_vec()) {
            // println!("{report:?} is safe");
            sum += 1;
            continue;
        }
        for i in 0..report.len() {
            let mut r = report.clone();
            r.remove(i);
            // println!("{report:?} | {i:?} | {r:?}");
            if is_safe(r.to_vec()) {
                // println!("{r:?} is safe");
                sum += 1;
                break;
            }
        }
    }
    println!("Day 2 part 2: {sum}");
    sum
}

pub fn run() {
    let input = parse_to_matrix(fs::read_to_string("data/day2.txt").expect("Reading day2 input"));
    part1(&input);
    part2(&input);
}

const _TESTCASE: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

#[test]
fn p1() {
    assert_eq!(part1(&parse_to_matrix(_TESTCASE.to_string())), 2);
}

#[test]
fn p2() {
    assert_eq!(part2(&parse_to_matrix(_TESTCASE.to_string())), 4);
}
