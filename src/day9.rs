use std::fs;

fn chars_to_files(input: &str) -> Vec<(u32, u32)> {
    // Gross, surely there's a better way!?
    let mut chars: Vec<char> = input
        .lines()
        .find(|l| !l.is_empty())
        .unwrap()
        .chars()
        .rev() // so I can .pop()
        .collect();

    let mut files = Vec::new();
    loop {
        match chars.len() {
            0 => break,
            1 => files.push((chars.pop().unwrap().to_digit(10).unwrap(), 0)),
            _ => files.push((
                chars.pop().unwrap().to_digit(10).unwrap(),
                chars.pop().unwrap().to_digit(10).unwrap(),
            )),
        }
    }
    // println!("Files: {:?}", files);
    files
}

fn files_to_seq(files: &[(u32, u32)]) -> Vec<i32> {
    let mut seq = Vec::new();
    for (i, (full, free)) in files.iter().enumerate() {
        for _ in 0..(*full as usize) {
            seq.push(i as i32);
        }
        for _ in 0..(*free as usize) {
            seq.push(-1);
        }
    }

    seq
}

fn part1(input: &str) -> i64 {
    let files = chars_to_files(input);
    let mut seq = files_to_seq(&files);
    let mut revseq = seq.clone();
    let mut result = Vec::new();
    let n = files.iter().fold(0, |acc, (i, _)| acc + i);
    revseq.reverse();

    // println!("Seq: {seq:?}");
    // println!("RevSeq: {revseq:?}");

    loop {
        if result.len() == n as usize {
            break;
        }
        let front = revseq.pop().unwrap();
        if front < 0 {
            loop {
                let back = seq.pop().unwrap();
                if back >= 0 {
                    result.push(back);
                    break;
                }
            }
        } else {
            result.push(front);
            continue;
        }
    }

    // println!("Result: {result:?}");

    result
        .iter()
        .enumerate()
        .fold(0, |acc, (i, v)| acc + (i as i64 * *v as i64))
}

// fn part2(input: &str) -> i64 {
//     // let files = chars_to_files(input);
//     // Starting from the highest file IDs (rightmost), try exactly once
//     // To fit the file in the leftmost position
//     // If it can fit, write both to result
//     // Else don't move it
//
//     0
// }

const _TESTCASE: &str = "2333133121414131402";

pub fn run() {
    let input = fs::read_to_string("data/day9.txt").expect("Reading day9.txt");
    println!("Day 9 part 1: {}", part1(&input));
    // println!("Day 9 part 2 {}", part2(&input));
}

#[test]
fn p1() {
    assert_eq!(part1("12345"), 60);
    assert_eq!(part1(_TESTCASE), 1928);
}
// #[test]
// fn p2() {
//     assert_eq!(part2(_TESTCASE), 2858);
// }
