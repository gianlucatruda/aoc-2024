use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

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

fn get_coord_dic(data: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut dic: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i, row) in data.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '.' {
                continue;
            }
            dic.entry(*c).or_default().push((i, j));
        }
    }
    dic
}

fn locate_antinodes(a: (usize, usize), b: (usize, usize)) -> Vec<(usize, usize)> {
    let mut res = Vec::new();

    let left = (
        a.0 as isize - (b.0 as isize - a.0 as isize),
        a.1 as isize - (b.1 as isize - a.1 as isize),
    );
    if left.0 >= 0 && left.1 >= 0 {
        res.push((left.0 as usize, left.1 as usize));
    }

    let right = (
        b.0 as isize + (b.0 as isize - a.0 as isize),
        b.1 as isize + (b.1 as isize - a.1 as isize),
    );
    if right.0 >= 0 && right.1 >= 0 {
        res.push((right.0 as usize, right.1 as usize));
    }

    res
}

fn print_antinode_grid(grid: &Vec<Vec<char>>, uniqs: &HashSet<(usize, usize)>) {
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if uniqs.contains(&(i, j)) {
                print!("#");
            } else {
                print!("{}", *c);
            }
        }
        println!();
    }
}

fn part1(input: &str) -> i32 {
    let mut uniqs = HashSet::new();
    let grid = parse_data(input);
    let (m, n) = (grid.len(), grid[0].len());
    let coords = get_coord_dic(&grid);
    // println!("{coords:?}");
    for (c, locs) in coords.iter() {
        // println!("Considering {c} ...");
        for pair in locs.iter().combinations(2) {
            // println!("Considering: {:?} vs {:?}", *pair[0], *pair[1]);
            let anodes = locate_antinodes(*pair[0], *pair[1]);
            for an in anodes.iter() {
                if an.0 < m && an.1 < n {
                    uniqs.insert(*an);
                    // println!("Antenode ({}): {:?}", *c, an);
                    // print_antinode_grid(&grid, &uniqs);
                }
            }
        }
    }

    uniqs.len().try_into().unwrap()
}

pub fn run() {
    assert_eq!(locate_antinodes((3, 4), (5, 5)), vec![(1, 3), (7, 6)]);
    assert_eq!(part1(_TESTCASE), 14);
    let input = fs::read_to_string("data/day8.txt").expect("Read day8.txt");
    let a = part1(&input);
    println!("Day 8 part 1: {a}");
}

#[test]
fn p1() {
    assert_eq!(part1(_TESTCASE), 14);
}
