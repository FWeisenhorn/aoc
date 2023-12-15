pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
// pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;

pub fn run(day: u8) {
    match day {
        1 => day01::run(),
        2 => day02::run(),
        3 => day03::run(),
        4 => day04::run(),
        5 => day05::run(),
        6 => day06::run(),
        7 => day07::run(),
        8 => day08::run(),
        9 => day09::run(),
        10 => day10::run(),
        11 => day11::run(),
        // 12 => day12::run(),
        13 => day13::run(),
        14 => day14::run(),
        15 => day15::run(),
        _ => println!("Invalid day!"),
    }
}
