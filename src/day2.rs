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

fn part1(data: Vec<Vec<i32>>) -> u32 {
    let mut sum = 0;
    for vals in data.iter() {
        let mut ds: Vec<i32> = Vec::new();
        for (i, v) in vals.iter().enumerate() {
            if i == vals.len() - 1 {
                break;
            }
            ds.push(vals[i + 1] - v);
        }
        let max = ds.iter().max().unwrap();
        let min = ds.iter().min().unwrap();

        // Utter dogshit
        if 1 <= max.abs() && max.abs() <= 3 && 1 <= min.abs() && min.abs() <= 3 {
            // What the fuck? Why do I have to dereference here?!?!?
            if *max < 0 && *min < 0 || *max > 0 && *min > 0 {
                sum += 1;
            }
        }
    }
    sum
}

// fn part2(data: Vec<Vec<i32>>) -> u32 {
//     0
// }

pub fn run() {
    let input = parse_to_matrix(fs::read_to_string("data/day2.txt").expect("Reading day2 input"));
    let a = part1(input);
    println!("Day 2 part 1: {a:}");
    println!("Day 2 part 2");
    // assert_eq!(part2(parse_to_matrix(TEST.to_string())), 4);
}

#[test]
fn p1() {
    assert_eq!(part1(parse_to_matrix(TEST.to_string())), 2);
}
