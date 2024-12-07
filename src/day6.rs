use std::{collections::HashMap, fs};

fn parse_grid_guard(input: &str) -> (Vec<Vec<char>>, (isize, isize)) {
    let grid: Vec<Vec<_>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let mut guard = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '^' {
                guard = (i as isize, j as isize);
                break;
            }
        }
    }

    (grid, guard)
}

fn log_route(grid: &[Vec<char>], guard: (isize, isize)) -> HashMap<(isize, isize), i32> {
    let mut guard = guard.clone();
    let (m, n) = (grid.len() as isize, grid[0].len() as isize);
    let mut logs = HashMap::new();
    logs.insert(guard, 1); // Starting positions counts
    let dirs = [
        (-1, 0), // Up
        (0, 1),  // Right
        (1, 0),  // Down
        (0, -1), // Left
    ];

    let mut dir = 0;
    loop {
        let ahead = (guard.0 + dirs[dir].0, guard.1 + dirs[dir].1);
        if !(0..m).contains(&ahead.0) || !(0..n).contains(&ahead.1) {
            break;
        }
        if grid[ahead.0 as usize][ahead.1 as usize] == '#' {
            // Turn 90 clockwise
            dir = (dir + 1) % 4;
        } else {
            guard = (ahead.0, ahead.1);
            // Increment log for that position
            match logs.get(&guard) {
                Some(&v) => logs.insert(guard, v + 1),
                _ => logs.insert(guard, 1),
            };
            // println!("Gaurd after step: {guard:?}");
        }
    }

    let total: i32 = logs.iter().map(|(_, v)| v).sum();
    // println!("Total moves: {total}");
    logs
}

fn part1(input: &str) -> i32 {
    let (grid, guard) = parse_grid_guard(input);
    let logs = log_route(&grid, guard);
    logs.len().try_into().unwrap()
}

fn part2(input: &str) -> i32 {
    // let (grid, guard) = parse_grid_guard(input);
    // let logs = log_route(&grid, guard);
    // logs.len().try_into().unwrap()
    0
}

const _TESTCASE: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

pub fn run() {
    let input = fs::read_to_string("data/day6.txt").expect("Reading day6.txt");
    let a = part1(&input);
    println!("Day 6 part 1: {a}");
    // assert_eq!(part2(_TESTCASE), 6);
}

#[test]
fn p1() {
    assert_eq!(part1(_TESTCASE), 41);
}
