use std::fs;

const _TESTCASE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

fn part1(input: &str) -> i32 {
    let mut sum: i32 = 0;
    let s = input.split("mul(").collect::<Vec<&str>>();
    for hit in s.iter() {
        let mut parts = hit.split(")").collect::<Vec<&str>>();
        // println!("{parts:?}");
        parts = parts[0].split(",").collect::<Vec<&str>>();
        if parts.len() != 2 {
            continue;
        }
        if parts[0].len() > 3 || parts[1].len() > 3 {
            continue;
        }
        let left: u16 = parts[0].parse().expect("Parse left value");
        let right: u16 = parts[1].parse().expect("Parse right value");

        // println!("{parts:?}\t{left} x {right}");
        if left > 999 || left < 1 || right > 999 || right < 1 {
            continue;
        }
        sum += ((left as i32) * (right as i32)) as i32;
    }

    sum
}

pub fn run() {
    assert_eq!(part1(_TESTCASE), 161);
    let p1 = part1(&fs::read_to_string("data/day3.txt").expect("Parse day3.txt"));
    println!("Day 3 part 1: {p1}");
}
