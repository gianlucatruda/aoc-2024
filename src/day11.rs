fn part1(input: &str, blinks: u32) -> i32 {
    let mut stones: Vec<u32> = input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Stones: {stones:?}");

    0
}

const _EXAMPLE: &str = "0 1 10 99 999";
const _TESTCASE: &str = "125 17";

pub fn run() {
    println!("\n\nDay 11\n");
    assert_eq!(part1(_EXAMPLE, 1), 7);
    assert_eq!(part1(_TESTCASE, 6), 22);
    assert_eq!(part1(_TESTCASE, 25), 55312);
}
