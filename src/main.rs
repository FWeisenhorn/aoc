#![warn(
    clippy::all,
    // clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    // clippy::cargo,
)]

mod aoc2023;

const YEAR: u32 = 2023;
const DAY: u32 = 10;

fn main() {
    match YEAR {
        2023 => aoc2023::run(DAY),
        _ => println!("Invalid Year!"),
    };
}
