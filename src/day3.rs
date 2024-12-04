use std::fs;

fn part1(input: &str) -> i32 {
    let mut sum: i32 = 0;
    let s = input.split("mul(").collect::<Vec<&str>>();
    for hit in s.iter() {
        let parts = hit.split(")").collect::<Vec<&str>>();

        if let Some((left, right)) = parts[0].split_once(",") {
            if let (Ok(l), Ok(r)) = (left.parse::<u16>(), right.parse::<u16>()) {
                if !(1..=999).contains(&r) || !(1..=999).contains(&l) {
                    continue;
                }
                sum += (l as i32) * (r as i32);
            } else {
                continue;
            }
        } else {
            continue;
        }

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
