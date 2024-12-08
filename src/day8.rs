use std::{collections::HashMap, fs};

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

fn get_coord_dic(data: Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut dic: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i, row) in data.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '.' {
                continue;
            }
            dic.entry(*c).or_insert_with(Vec::new).push((i, j));
        }
    }
    dic
}

fn part1(input: &str) -> i32 {
    let grid = parse_data(input);
    let coords = get_coord_dic(grid);
    // println!("{coords:?}");
    0
}

pub fn run() {
    println!("\nday 8...");
    assert_eq!(part1(_TESTCASE), 14);
    let input = fs::read_to_string("data/day8.txt").expect("Read day8.txt");
    let a = part1(&input);
    println!("Day 8 part 1: {a}");
}
