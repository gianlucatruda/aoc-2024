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
            dic.entry(*c).or_insert_with(Vec::new).push((i, j));
        }
    }
    dic
}

fn locate_antinodes(a: (usize, usize), b: (usize, usize)) -> Vec<(usize, usize)> {
    let dy = a.0.abs_diff(b.0);
    let dx = a.1.abs_diff(b.1);
    let mut res = Vec::new();
    if a.0 as isize - dy as isize > 0 && a.1 as isize - dx as isize > 0 {
        res.push((a.0 - dy, a.1 - dx));
    }
    res.push((b.0 + dy, b.1 + dx));

    // vec![(a.0 - dy, a.1 - dx), (b.0 + dy, b.1 + dx)]
    res
}

fn part1(input: &str) -> i32 {
    let mut uniqs = HashSet::new();
    let grid = parse_data(input);
    let (m, n) = (grid.len(), grid[0].len());
    let coords = get_coord_dic(&grid);
    // println!("{coords:?}");
    for (c, locs) in coords.iter() {
        println!("Considering {c} ...");
        let pairs: Vec<_> = locs.into_iter().combinations(2).collect();
        println!("{pairs:?}");
        for pair in pairs.iter() {
            println!("Considering: {:?} vs {:?}", *pair[0], *pair[1]);
            let anodes = locate_antinodes(*pair[0], *pair[1]);
            for an in anodes.iter() {
                if an.0 < m && an.1 < n {
                    uniqs.insert(an.clone());
                    println!("Antenode ({}): {:?}", *c, an);
                }
            }
        }
    }

    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if uniqs.contains(&(i, j)) {
                print!("#");
            } else {
                print!("{}", *c);
            }
        }
        print!("\n");
    }

    uniqs.len().try_into().unwrap()
}

pub fn run() {
    println!("\nday 8...");
    assert_eq!(locate_antinodes((3, 4), (5, 5)), vec![(1, 3), (7, 6)]);
    assert_eq!(part1(_TESTCASE), 14);
    let input = fs::read_to_string("data/day8.txt").expect("Read day8.txt");
    let a = part1(&input);
    println!("Day 8 part 1: {a}");
}
