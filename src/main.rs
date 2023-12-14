#![warn(
    clippy::all,
    // clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    // clippy::cargo,
)]

mod aoc2023;

const YEAR: u8 = 23;
const DAY: u8 = 11;

fn main() {
    match YEAR {
        23 => aoc2023::run(DAY),
        _ => println!("Invalid Year!"),
    };
}
