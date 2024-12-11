use std::fs;

fn part1(input: &str, blinks: u32) -> i32 {
    let mut stones: Vec<u64> = input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Stones: {stones:?}");

    for _ in 0..blinks {
        let mut new: Vec<u64> = Vec::new();
        for s in stones {
            let digits = (s as f64).log10() as u32 + 1;
            if digits % 2 == 0 {
                let s_str = s.to_string();
                let left_str: String = s_str.chars().take((digits / 2) as usize).collect();
                let left: u64 = left_str.parse().unwrap();
                let right_str: String = s_str.chars().rev().take((digits / 2) as usize).collect();
                let right: u64 = right_str.chars().rev().collect::<String>().parse().unwrap();
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
    println!("\n\nDay 11\n");
    assert_eq!(part1(_EXAMPLE, 1), 7);
    assert_eq!(part1(_TESTCASE, 6), 22);
    assert_eq!(part1(_TESTCASE, 25), 55312);
    let input = fs::read_to_string("data/day11.txt").expect("Reading day11.txt");
    println!("Day 11 part 1: {}", part1(&input, 25));
}
