use std::{fmt, fs};

type Stream = Vec<char>;

#[derive(Debug)]
struct Disk {
    stream: Stream,
}

impl Disk {
    fn to_str(&self) -> String {
        self.stream.iter().collect()
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let st = self.to_str();
        write!(f, "{}", st)
    }
}

fn chars_to_files(input: &str) -> Vec<(u32, u32)> {
    // Gross, surely there's a better way!?
    let mut chars: Vec<char> = input
        .lines()
        .filter(|l| !l.is_empty())
        .next()
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
    // println!("{:?}", files);
    files
}

fn files_to_seq(files: Vec<(u32, u32)>) -> Disk {
    let mut seq: Vec<char> = Vec::new();
    for (fid, (flen, ffree)) in files.iter().enumerate() {
        // println!("{fid:?} | {flen:?} | {ffree:?}");
        for _ in 0..(*flen) {
            if let Some(c) = std::char::from_digit(fid as u32, 10) {
                seq.push(c);
            }
        }
        for _ in 0..(*ffree) {
            seq.push('.');
        }
    }
    Disk { stream: seq }
}

fn compact_seq(mut seq: Disk) -> Disk {
    let n = seq.to_str().len();
    loop {
        if let Some(free_idx) = seq.to_str().find('.') {
            if seq
                .stream
                .get(free_idx..)
                .unwrap()
                .into_iter()
                .all(|c| *c == '.')
            {
                break;
            }
            if let Some(c) = seq.stream.pop() {
                // println!("Popped {}", c);
                if c != '.' {
                    seq.stream[free_idx] = c;
                }
                // seq.stream.push('.');
                // println!("{:?}", seq.to_str().get(n - 50..));
            }
        } else {
            break;
        }
    }

    while seq.stream.len() < n {
        seq.stream.push('.');
    }

    seq
}

fn part1(input: &str) -> i32 {
    let files = chars_to_files(input);
    let mut seq = files_to_seq(files);
    // println!("{:?}", seq);
    seq = compact_seq(seq);
    let mut sum = 0;
    for (i, c) in seq.stream.iter().enumerate() {
        if *c != '.' {
            sum += i as u32 * c.to_digit(10).unwrap();
        }
    }
    sum.try_into().unwrap()
}

const _TESTCASE: &str = "2333133121414131402";

pub fn run() {
    println!("\n\nday 9:\n");
    assert_eq!(
        files_to_seq(chars_to_files("12345")).to_str(),
        "0..111....22222"
    );
    assert_eq!(
        files_to_seq(chars_to_files("2333133121414131402")).to_str(),
        "00...111...2...333.44.5555.6666.777.888899"
    );
    assert_eq!(
        compact_seq(files_to_seq(chars_to_files("12345"))).to_str(),
        "022111222......"
    );
    assert_eq!(
        compact_seq(files_to_seq(chars_to_files("2333133121414131402"))).to_str(),
        "0099811188827773336446555566.............."
    );
    assert_eq!(part1(_TESTCASE), 1928);
    assert_eq!(part1("12345"), 60);

    let qc = part1("5365105859596839658113");
    println!("QC: {qc}");

    let input = fs::read_to_string("data/day9.txt").expect("Reading day9.txt");
    let a = part1(&input);
    println!("Day 9 part 1: {a}");
}
