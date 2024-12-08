use std::fs;

const _TESTCASE: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............

";

fn parse_data(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect()
}

fn part1(input: &str) -> i32 {
    let grid = parse_data(input);
    println!("{grid:?}");
    0
}

pub fn run() {
    println!("\nday 8...");
    assert_eq!(part1(_TESTCASE), 14);
    let input = fs::read_to_string("data/day8.txt").expect("Read day8.txt");
    let a = part1(&input);
    println!("Day 8 part 1: {a}");
}