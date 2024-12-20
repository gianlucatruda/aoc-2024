use std::time::Instant;

mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn run_day(d: u32) {
    let func = match d {
        1 => day1::run,
        2 => day2::run,
        3 => day3::run,
        4 => day4::run,
        5 => day5::run,
        6 => day6::run,
        7 => day7::run,
        8 => day8::run,
        9 => day9::run,
        10 => day10::run,
        11 => day11::run,
        12 => day12::run,
        _ => {
            println!("Invalid day");
            return;
        }
    };

    let start = Instant::now();
    func();
    let duration = start.elapsed();
    println!("{:02}.{:03}s", duration.as_secs(), duration.subsec_millis(),);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day: u32 = args[1].parse().expect("Please provide a valid day number");
    run_day(day);
}
