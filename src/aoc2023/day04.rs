use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("inputs/day04.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    input
        .lines()
        .map(calc_wins_per_line)
        .map(|n| match n {
            0 => 0,
            x => 2u32.pow(u32::try_from(x).unwrap()) / 2,
        })
        .sum::<u32>()
        .to_string()
}

fn part_b(input: &str) -> String {
    input
        .lines()
        .map(calc_wins_per_line)
        .fold(
            (0usize, VecDeque::<usize>::new()),
            |mut acc: (usize, VecDeque<usize>), x| {
                let cur = 1 + acc.1.pop_front().unwrap_or(0);
                acc.0 += cur;
                for k in 0..x {
                    match acc.1.get_mut(k) {
                        Some(q) => *q += cur,
                        None => acc.1.push_back(cur),
                    }
                }
                acc
            },
        )
        .0
        .to_string()
}

fn calc_wins_per_line(line: &str) -> usize {
    let x = line.split_once(':').unwrap();
    let x = x.1.split_once('|').unwrap();

    let winning_numbers: HashSet<u32> =
        x.0.split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

    x.1.split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .filter(|x| winning_numbers.contains(x))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day04.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), "13");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), "30");
    }
}
