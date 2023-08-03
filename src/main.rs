mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod file;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    let filename = &args[2];

    match day.parse::<u8>().unwrap_or(0) {
        1 => day1::solution(filename),
        2 => day2::solution(filename),
        3 => day3::solution(filename),
        4 => day4::solution(filename),
        5 => day5::solution(filename),
        6 => day6::solution(filename),
        7 => day7::solution(filename),
        _ => panic!("unknown day number"),
    }
}
