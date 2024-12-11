use std::fs;

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

fn part2(input: &str, blinks: u32) -> i32 {
    let mut stones = parse_to_stones(input);
    // println!("Stones: {stones:?}");

    for b in 0..blinks {
        println!("Blink {} of {}", b, blinks);
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
        // println!("Update: {stones:?}");
    }
    stones.len().try_into().unwrap()
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
