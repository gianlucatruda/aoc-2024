use std::{collections::HashMap, fs};

fn parse_to_stones(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1(input: &str, blinks: u32) -> i32 {
    let mut stones = parse_to_stones(input);

    for _ in 0..blinks {
        let mut new: Vec<u64> = Vec::new();
        for s in stones {
            let digits = (s as f64).log10() as u32 + 1;
            if digits % 2 == 0 {
                let left: u64 = s / 10_u64.pow(digits / 2);
                let right: u64 = s % 10_u64.pow(digits / 2);
                new.push(left);
                new.push(right);
            } else {
                match s {
                    0 => new.push(1),
                    _ => new.push(s * 2024),
                }
            }
        }
        stones = new;
    }
    stones.len().try_into().unwrap()
}

fn part2(input: &str, blinks: usize) -> usize {
    let stones = parse_to_stones(input);

    // Histogram of values for efficiency
    let mut stone_counts: HashMap<u64, usize> = HashMap::new();
    for s in stones.iter() {
        *stone_counts.entry(*s).or_insert(0) += 1;
    }

    for _ in 0..blinks {
        let mut new_counts: HashMap<u64, usize> = HashMap::new();
        for (stone, count) in stone_counts {
            if stone == 0 {
                // All stones with value 0 => 1
                *new_counts.entry(1).or_insert(0) += count;
            } else if stone.to_string().len() % 2 == 0 {
                // Even num. digits split
                let newlens = stone.to_string().len() as u32 / 2;
                let left = stone / 10_u64.pow(newlens);
                let right = stone % 10_u64.pow(newlens);
                *new_counts.entry(left).or_insert(0) += count;
                *new_counts.entry(right).or_insert(0) += count;
            } else {
                // Else multiply by 2024
                let new_stone = &stone * 2024;
                *new_counts.entry(new_stone).or_insert(0) += count;
            }
        }
        stone_counts = new_counts;
    }
    stone_counts.values().sum()
}

const _EXAMPLE: &str = "0 1 10 99 999";
const _TESTCASE: &str = "125 17";

pub fn run() {
    let input = fs::read_to_string("data/day11.txt").expect("Reading day11.txt");
    println!("Day 11 part 1: {}", part1(&input, 25));
    println!("Day 11 part 2 {}", part2(&input, 75));
}

#[test]
fn p1() {
    assert_eq!(part1(_EXAMPLE, 1), 7);
    assert_eq!(part1(_TESTCASE, 6), 22);
    assert_eq!(part1(_TESTCASE, 25), 55312);
}
#[test]
fn p2() {
    assert_eq!(part2(_EXAMPLE, 1), 7);
    assert_eq!(part2(_TESTCASE, 6), 22);
    assert_eq!(part2(_TESTCASE, 25), 55312);
}
