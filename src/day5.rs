// https://adventofcode.com/2024/day/5

fn parse_inp(input: &str) -> (Vec<(u16, u16)>, Vec<Vec<u16>>) {
    let (rstr, ustr) = input.split_once("\n\n").expect("Parsing input");

    let rules: Vec<(u16, u16)> = rstr
        .lines()
        .filter_map(|l| l.split_once("|"))
        .filter_map(|(l, r)| Some((l.parse::<u16>().unwrap(), r.parse::<u16>().unwrap())))
        .collect();

    let updates: Vec<Vec<u16>> = ustr
        .lines()
        .filter_map(|line| {
            if !line.is_empty() {
                Some(line.split(',').map(|s| s.parse().unwrap()).collect())
            } else {
                None
            }
        })
        .collect();

    (rules, updates)
}

fn part1(input: &str) -> i32 {
    let (rules, updates) = parse_inp(input);
    0
}

const _TESTCASE: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47

";

pub fn run() {
    println!("day5");
    assert_eq!(part1(_TESTCASE), 143);
}
