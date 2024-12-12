fn part1(input: &str) -> i32 {
    0
}

const _TESTCASE: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

const _EXAMPLE1: &str = "\
AAAA
BBCD
BBCC
EEEC
";

const _EXAMPLE2: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

pub fn run() {
    println!("\n\nday12\n");
    assert_eq!(part1(_TESTCASE), 1930);
    assert_eq!(part1(_EXAMPLE1), 140);
    assert_eq!(part1(_EXAMPLE2), 772);
}
