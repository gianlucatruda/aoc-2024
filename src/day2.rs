use std::fs;
const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn parse_to_matrix(input: String) -> Vec<Vec<i32>> {
    let mut data: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        // Account for empty lines (e.g. at EoF)
        if line.len() < 1 {
            continue;
        }
        let vals: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        data.push(vals);
    }

    data
}

fn is_safe(report: Vec<i32>) -> bool {
    let mut ds: Vec<i32> = Vec::new();
    for (i, v) in report.iter().enumerate() {
        if i == report.len() - 1 {
            break;
        }
        ds.push(report[i + 1] - v);
    }
    let max = ds.iter().max().unwrap();
    let min = ds.iter().min().unwrap();

    // Utter dogshit
    if 1 <= max.abs() && max.abs() <= 3 && 1 <= min.abs() && min.abs() <= 3 {
        // What the fuck? Why do I have to dereference here?!?!?
        if *max < 0 && *min < 0 || *max > 0 && *min > 0 {
            return true;
        }
    }
    false
}

fn part1(data: Vec<Vec<i32>>) -> u32 {
    let mut sum = 0;
    for vals in data.iter() {
        if is_safe(vals.to_vec()) {
            sum += 1;
        }
    }
    sum
}

fn part2(data: Vec<Vec<i32>>) -> u32 {
    let mut sum = 0;
    for report in data.iter() {
        if is_safe(report.to_vec()) {
            // println!("{report:?} is safe");
            sum += 1;
            continue;
        }
        for i in 0..5 {
            let mut r = report.clone();
            r.remove(i);
            // println!("{report:?} | {i:?} | {r:?}");
            if is_safe(r.to_vec()) {
                // println!("{r:?} is safe");
                sum += 1;
                break;
            }
        }

    }
    sum
}

pub fn run() {
    let input = parse_to_matrix(fs::read_to_string("data/day2.txt").expect("Reading day2 input"));
    let a = part1(input.clone());
    println!("Day 2 part 1: {a}");
    let b = part2(input.clone());
    println!("Day 2 part 2: {b}");
}

#[test]
fn p1() {
    assert_eq!(part1(parse_to_matrix(TEST.to_string())), 2);
}

#[test]
fn p2() {
    assert_eq!(part2(parse_to_matrix(TEST.to_string())), 4);
}
