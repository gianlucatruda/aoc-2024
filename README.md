# Advent of Code challenge 2024

https://adventofcode.com/2024

Disclaimer: I [started learning Rust](https://gianluca.ai/tags/rust/) a few months ago and am using AoC as an opportunity to practice and hone intuitions.

## Progress log

- [x] Day 1
    - [x] Part 1
    - [x] Part 2
    - [ ] Optimised
- [x] Day 2
    - [x] Part 1
    - [x] Part 2
    - [ ] Optimised
- [x] Day 3
    - [x] Part 1
    - [x] Part 2
    - [x] Optimised
- [x] Day 4
    - [x] Part 1
    - [x] Part 2
    - [x] Optimised
- [x] Day 5
    - [x] Part 1
    - [x] Part 2
    - [x] Optimised
- [x] Day 6
    - [x] Part 1
    - [x] Part 2 (~15s in dev, ~1.1s in release)
    - [x] Optimised (I tried, at least)
- [x] Day 7
    - [x] Part 1
    - [x] Part 2
    - [x] Optimised (more efficient `cat` for ~4x speedup)
    - [ ] Maybe: Write own permutations func to replace itertools dep
- [x] Day 8
    - [x] Part 1
    - [x] Part 2
    - [x] Optimised (didn't really need it)
- [ ] Day 9
    - [ ] Part 1
    - [ ] Part 2


## Notes

I'm explicitly telling `main.rs` which other filenames (e.g. `day1.rs`) to consider modules and import (e.g. `mod day1;`) and then calling the public `run()` function for each, which prints the answers using the input from `data/day1.txt` (which is different every day and unique per user).

TODO: look into profiling tools for Rust. Line profiling? How can I see if I'm actually making my code better/faster once I have it working?

## Usage

Run test suite (on the example cases):

```bash
cargo test
```

Run solutions for a given day (requires inputs in `data/`):

```bash
cargo run <day> --release
```
