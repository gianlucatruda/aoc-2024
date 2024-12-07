use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn run_day(d: u32) {
    let func = match d {
        1 => day1::run,
        2 => day2::run,
        3 => day3::run,
        4 => day4::run,
        5 => day5::run,
        6 => day6::run,
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
