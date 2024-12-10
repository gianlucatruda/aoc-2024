use std::{collections::HashSet, fs};

type Loc = (usize, usize);
type Grid = Vec<Vec<u32>>;
const DIRS: [(isize, isize); 4] = [
    (-1, 0), // Up
    (0, 1),  // Right
    (1, 0),  // Down
    (0, -1), // Left
];

fn parse_to_grid(input: &str) -> Grid {
    input
        .trim()
        .lines()
        .map(|l: &str| l.chars().map(|c: char| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn find_trailheads(grid: &Grid) -> Vec<Loc> {
    let mut trailheads: Vec<Loc> = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            if *v == 0 {
                trailheads.push((i, j));
            }
        }
    }
    trailheads
}

fn reachable_peaks((i0, j0): Loc, grid: &Grid) -> HashSet<Loc> {
    let this = grid[i0][j0];
    if this == 9 {
        return HashSet::from([(i0, j0)]);
    }
    let mut peaks: HashSet<Loc> = HashSet::new();
    let m = grid.len() as isize;
    for (di, dj) in DIRS.iter() {
        let (i, j) = (i0 as isize + di, j0 as isize + dj);
        if i < 0 || i >= m || j < 0 || j >= m {
            continue;
        }
        let next = grid[i as usize][j as usize];
        if next == this + 1 {
            let reachable = reachable_peaks((i as usize, j as usize), grid);
            peaks.extend(reachable);
        }
    }
    peaks
}

fn distinct_routes((i0, j0): Loc, grid: &Grid) -> u32 {
    let this = grid[i0][j0];
    if this == 9 {
        return 1;
    }
    let mut peaks = 0;
    let m = grid.len() as isize;
    for (di, dj) in DIRS.iter() {
        let (i, j) = (i0 as isize + di, j0 as isize + dj);
        if i < 0 || i >= m || j < 0 || j >= m {
            continue;
        }
        let next = grid[i as usize][j as usize];
        if next == this + 1 {
            peaks += distinct_routes((i as usize, j as usize), grid);
        }
    }
    peaks
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    let grid = parse_to_grid(input);
    let trailheads = find_trailheads(&grid);
    for (i0, j0) in trailheads.iter() {
        let score = reachable_peaks((*i0, *j0), &grid).len();
        sum += score;
        // println!("Trailhead: {i0}, {j0}\tscore: {score}");
    }
    sum.try_into().unwrap()
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;
    let grid = parse_to_grid(input);
    let trailheads = find_trailheads(&grid);
    for (i0, j0) in trailheads.iter() {
        let score = distinct_routes((*i0, *j0), &grid);
        sum += score;
        // println!("Trailhead: {i0}, {j0}\tscore: {score}");
    }
    sum.try_into().unwrap()
}

pub fn run() {
    let input = fs::read_to_string("data/day10.txt").expect("Reading day10.txt");
    let a = part1(&input);
    println!("Day 10 part 1: {a}");
    let b = part2(&input);
    println!("Day 10 part 2: {b}");
}

const _TESTCASE: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

#[test]
fn p1() {
    assert_eq!(part1(_TESTCASE), 36);
}

#[test]
fn p2() {
    assert_eq!(part2(_TESTCASE), 81);
}
