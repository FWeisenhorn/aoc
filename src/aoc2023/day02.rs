use std::cmp::max;

const INPUT: &str = include_str!("inputs/day02.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    input
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .filter_map(|(a, b)| {
            if b.split(&[',', ';'])
                .any(|s| match s[1..].split_once(' ').unwrap() {
                    (i, "red") => i.parse::<u32>().unwrap() > 12,
                    (i, "green") => i.parse::<u32>().unwrap() > 13,
                    (i, "blue") => i.parse::<u32>().unwrap() > 14,
                    _ => unreachable!(),
                })
            {
                None
            } else {
                a.split_whitespace().last().unwrap().parse::<u32>().ok()
            }
        })
        .sum::<u32>()
        .to_string()
}

pub fn part_b(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split(&[',', ';'])
                .fold([0u32; 3], |acc, s| {
                    let mut x = acc;
                    match s[1..].split_once(' ').unwrap() {
                        (i, "red") => x[0] = max(acc[0], i.parse().unwrap()),
                        (i, "blue") => x[1] = max(acc[1], i.parse().unwrap()),
                        (i, "green") => x[2] = max(acc[2], i.parse().unwrap()),
                        _ => unreachable!(),
                    };
                    x
                })
                .iter()
                .product::<u32>()
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day02.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), "8");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), "2286");
    }
}
