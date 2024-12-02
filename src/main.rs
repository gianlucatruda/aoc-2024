use std::{fs, iter::zip};

fn parse_and_sort_lists() -> (Vec<u32>, Vec<u32>) {
    let input = fs::read_to_string("input.txt").expect("Read input");
    let mut a: Vec<u32> = Vec::new();
    let mut b: Vec<u32> = Vec::new();

    for line in input.lines() {
        // println!("{line}");
        let data: Vec<&str> = line.split_whitespace().collect();
        if data.len() < 2 {
            continue;
        }
        // println!("{data:?}");
        let left: u32 = data[0].parse().expect("str to int");
        let right: u32 = data[1].parse().expect("str to int");
        // println!("{left} vs {right}");

        a.push(left);
        b.push(right);
    }

    a.sort();
    b.sort();
    // println!("A: {a:?}");
    // println!("B: {b:?}");

    (a, b)
}

fn part1(a: &Vec<u32>, b: &Vec<u32>) {
    let mut total: u32 = 0;
    for (l, r) in zip(a, b) {
        let val = match l >= r {
            true => l - r,
            _ => r - l,
        };
        // println!("{val}");
        total += val;
    }
    println!("Day 1 part 1: {total}");
}

fn part2(a: &Vec<u32>, b: &Vec<u32>) {
    let mut score: u32 = 0;
    for av in a.iter() {
        let mut hits = 0;
        for bv in b.iter() {
            if av == bv {
                hits += 1;
            }
        }
        score += av * hits;
    }
    println!("Day 1 part 2: {score}");
}

fn main() {
    let (a, b) = parse_and_sort_lists();
    part1(&a, &b);
    part2(&a, &b);
}
