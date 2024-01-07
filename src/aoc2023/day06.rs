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
        .filter_map(|x| x.parse::<u32>().ok());
    let dists = input[1]
        .split_whitespace()
        .skip(1)
        .filter_map(|x| x.parse::<u32>().ok());

    times
        .zip(dists)
        .map(|(t, d)| (0..t).filter(|i| (t - i) * i > d).count())
        .product::<usize>()
        .to_string()
}

fn part_b(input: &str) -> String {
    let input: Vec<&str> = input.lines().collect();

    let time = input[0]
        .split_whitespace()
        .skip(1)
        .flat_map(str::chars)
        .collect::<String>()
        .parse::<u64>()
        .ok();

    let dist = input[1]
        .split_whitespace()
        .skip(1)
        .flat_map(str::chars)
        .collect::<String>()
        .parse::<u64>()
        .ok();

    time.zip(dist)
        .and_then(|(t, d)| min_and_max_solutions(t, d))
        .map_or_else(|| unreachable!(), |x| x.to_string())
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

    #[test]
    fn result_a() {
        assert_eq!(part_a(INPUT), "4568778");
    }

    #[test]
    fn result_b() {
        assert_eq!(part_b(INPUT), "28973936");
    }
}
