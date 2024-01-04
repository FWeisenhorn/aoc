use std::ops::Div;

const INPUT: &str = include_str!("inputs/day06.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    let input: Vec<&str> = input.lines().collect();

    let times = input[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u32>().unwrap());
    let dists = input[1]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u32>().unwrap());

    times
        .zip(dists)
        .map(|(t, d)| (0..t).filter(|i| (t - i) * i > d).count())
        .product::<usize>()
        .to_string()
}

fn part_b(input: &str) -> String {
    let input: Vec<&str> = input.lines().collect();

    let time: u64 = input[0]
        .split_whitespace()
        .skip(1)
        .flat_map(str::chars)
        .collect::<String>()
        .parse()
        .unwrap();

    let dist: u64 = input[1]
        .split_whitespace()
        .skip(1)
        .flat_map(str::chars)
        .collect::<String>()
        .parse()
        .unwrap();

    min_and_max_solutions(time, dist).unwrap().to_string()
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
fn min_and_max_solutions(t: u64, d: u64) -> Option<u64> {
    let t2 = (t * t).checked_sub(4 * d)?;
    let x = (t2 as f64).sqrt();
    let x1 = ((t as f64 - x).div(2.)).ceil() as u64;
    let x2 = ((t as f64 + x).div(2.)).floor() as u64;

    Some(x2 - x1 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day06.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), "288");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), "71503");
    }
}
