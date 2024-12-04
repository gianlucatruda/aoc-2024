use std::fs;

fn part1(input: &str) -> i32 {
    let mut sum: i32 = 0;
    let s = input.split("mul(").collect::<Vec<&str>>();
    for hit in s.iter() {
        let mut parts = hit.split(")").collect::<Vec<&str>>();
        parts = parts[0].split(",").collect::<Vec<&str>>();
        if parts.len() != 2 {
            continue;
        }
        if parts[0].len() > 3 || parts[1].len() > 3 {
            continue;
        }
        let left: u16 = parts[0].parse().expect("Parse left value");
        let right: u16 = parts[1].parse().expect("Parse right value");

        if left > 999 || left < 1 || right > 999 || right < 1 {
            continue;
        }
        sum += ((left as i32) * (right as i32)) as i32;
    }

    sum
}

fn part2(input: &str) -> i32 {
    let mut sum: i32 = 0;
    let mut inp = input;

    loop {
        if let Some((l, r)) = inp.split_once("don't()") {
            sum += part1(l);
            inp = r;
            if let Some((_, r)) = inp.split_once("do()") {
                inp = r;
            } else {
                return sum;
            }
        } else {
            sum += part1(inp);
            return sum;
        }
    }
}

pub fn run() {
    let input = &fs::read_to_string("data/day3.txt").expect("Parse day3.txt");
    let p1 = part1(input);
    println!("Day 3 part 1: {p1}");
    let p2 = part2(input);
    println!("Day 3 part 2: {p2}");
}

const _TESTCASE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const _TESTCASE_2: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

#[test]
fn p1() {
    assert_eq!(part1(_TESTCASE), 161);
}

#[test]
fn p2() {
    assert_eq!(part2(_TESTCASE_2), 48);
}
