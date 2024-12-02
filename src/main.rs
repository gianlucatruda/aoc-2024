use std::{fs, iter::zip};

fn main() {
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

    let mut total: u32 = 0;
    for (l, r) in zip(a, b) {
        let val = match l >= r {
            true => l - r,
            _ => r - l,
        };
        total += val;

        // println!("{val}");
    }
    println!("Answer: {total}");
}
