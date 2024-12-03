use std::fs;
const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        // Account for empty lines (e.g. at EoF)
        if line.len() < 1 {
            continue;
        }
        // println!("{line}");
        let vals: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        // println!("{vals:?}");

        let mut ds: Vec<i32> = Vec::new();
        for (i, v) in vals.iter().enumerate() {
            if i == vals.len() - 1 {
                break;
            }
            ds.push(vals[i + 1] - v);

            // TIL .abs and .abs_diff
        }
        // println!("{vals:?}\t{ds:?}");
        let max = ds.iter().max().unwrap();
        let min = ds.iter().min().unwrap();
        // println!("{max} {min}");

        // Utter dogshit
        if 1 <= max.abs() && max.abs() <= 3 && 1 <= min.abs() && min.abs() <= 3 {
            // What the fuck? Why do I have to dereference here?!?!?
            if *max < 0 && *min < 0 || *max > 0 && *min > 0 {
                // println!("{line:} {min:} {max:}");
                sum += 1;
            }
        }
    }
    sum
}

pub fn run() {
    let input = fs::read_to_string("data/day2.txt").expect("Reading day2 input");
    println!("day 2");
    let a = part1(&input);
    println!("Day 2 part 1: {a:}");
}

#[test]
fn p1() {
    assert_eq!(part1(TEST), 2);
}
