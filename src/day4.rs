use std::fs;
// https://adventofcode.com/2024/day/4

fn add_run_in_dir(data: &Vec<Vec<char>>, r: usize, c: usize, x: i32, y: i32) -> i32 {
    for (i, l) in ['M', 'A', 'S'].iter().enumerate() {
        let a = (r as i32) + (x * (i + 1) as i32);
        let b = (c as i32) + (y * (i + 1) as i32);
        let (m, n) = (data.len() as i32, data[0].len() as i32);
        if !(0..m).contains(&a) || !(0..n).contains(&b) {
            return 0;
        }
        if data[a as usize][b as usize] != *l {
            return 0;
        }
    }
    1
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    let data: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    for (r, row) in data.iter().enumerate() {
        for (c, char) in row.iter().enumerate() {
            if *char == 'X' {
                // It aint pretty but it's honest work
                // horizontal, vertical, diagonal, written backwards, or even overlapping other words
                sum += add_run_in_dir(&data, r, c, -1, -1);
                sum += add_run_in_dir(&data, r, c, -1, 0);
                sum += add_run_in_dir(&data, r, c, -1, 1);
                sum += add_run_in_dir(&data, r, c, 0, -1);
                sum += add_run_in_dir(&data, r, c, 0, 1);
                sum += add_run_in_dir(&data, r, c, 1, -1);
                sum += add_run_in_dir(&data, r, c, 1, 0);
                sum += add_run_in_dir(&data, r, c, 1, 1);
            }
        }
    }
    sum
}

const _TESTCASE: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

pub fn run() {
    let input = fs::read_to_string("data/day4.txt").expect("Read day4.txt");
    let a = part1(&input);
    println!("Day 4 part 1: {a}");
}

#[test]
fn p1() {
    assert_eq!(part1(_TESTCASE), 18);
}
