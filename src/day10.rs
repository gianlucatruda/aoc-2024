use std::fs;

type Loc = (usize, usize);

fn part1(input: &str) -> i32 {
    let grid: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|l: &str| l.chars().map(|c: char| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut trailheads: Vec<Loc> = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            if *v == 0 {
                trailheads.push((i, j));
            }
        }
    }
    println!("Trailheads found: {:?}\t{:?}", trailheads.len(), trailheads);

    for (i0, j0) in trailheads.iter() {
        println!("{i0}, {j0}");
    }

    0
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

pub fn run() {
    println!("\n\nDay 10\n");
    let input = fs::read_to_string("data/day10.txt").expect("Reading day10.txt");
    let a = part1(&input);
    println!("Day 10 part 1: {a}");
    assert_eq!(part1(_TESTCASE), 36);
}
