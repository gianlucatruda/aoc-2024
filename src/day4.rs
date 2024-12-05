use std::fs;
// https://adventofcode.com/2024/day/4

fn matrixify(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn part1(input: &str) -> i32 {
    let data = matrixify(input);
    let (m, n) = (data.len(), data[0].len());
    let directions = [
        (-1, -1), // Up-left
        (-1, 0),  // Up
        (-1, 1),  // Up-right
        (0, -1),  // Left
        (0, 1),   // Right
        (1, -1),  // Down-left
        (1, 0),   // Down
        (1, 1),   // Down-right
    ];
    let mut sum = 0;
    for (r, row) in data.iter().enumerate() {
        for (c, char) in row.iter().enumerate() {
            if *char != 'X' {
                continue;
            }
            for &(dx, dy) in &directions {
                // sum += add_run_in_dir(&data, r as i32, c as i32, dx as i32, dy as i32);
                for (i, l) in ['M', 'A', 'S'].iter().enumerate() {
                    let a = r as i32 + (dx * (i as i32 + 1));
                    let b = c as i32 + (dy * (i as i32 + 1));
                    if !(0..m as i32).contains(&a) || !(0..n as i32).contains(&b) {
                        // Index out of range
                        break;
                    }
                    if data[a as usize][b as usize] != *l {
                        // Character not in MAS run
                        break;
                    }
                    if i == 2 {
                        // Valid XMAS found
                        sum += 1;
                    }
                }
            }
        }
    }
    sum
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;
    let data = matrixify(input);
    for (r, row) in data.iter().enumerate().skip(1).take(data.len() - 2) {
        for (c, char) in row.iter().enumerate().skip(1).take(data[0].len() - 2) {
            if *char == 'A' {
                let mut arms = 0;
                // Up and left
                match data[r - 1][c - 1] {
                    'M' => {
                        // Down and right
                        match data[r + 1][c + 1] {
                            'S' => arms += 1,
                            _ => continue,
                        }
                    }
                    'S' => {
                        // Down and right
                        match data[r + 1][c + 1] {
                            'M' => arms += 1,
                            _ => continue,
                        }
                    }
                    _ => continue,
                }
                // Down and left
                match data[r + 1][c - 1] {
                    'M' => match data[r - 1][c + 1] {
                        'S' => arms += 1,
                        _ => continue,
                    },
                    'S' => match data[r - 1][c + 1] {
                        'M' => arms += 1,
                        _ => continue,
                    },
                    _ => continue,
                }
                if arms == 2 {
                    sum += 1;
                }
            }
        }
    }
    sum
}

const _TESTCASE: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

pub fn run() {
    assert_eq!(part2(_TESTCASE), 9);

    let input = fs::read_to_string("data/day4.txt").expect("Read day4.txt");
    let a = part1(&input);
    println!("Day 4 part 1: {a}");
    let b = part2(&input);
    println!("Day 4 part 2: {b}");
}

#[test]
fn p1() {
    assert_eq!(part1(_TESTCASE), 18);
}

#[test]
fn p2() {
    assert_eq!(part2(_TESTCASE), 9);
}
