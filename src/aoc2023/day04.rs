use std::collections::VecDeque;

const INPUT: &str = include_str!("inputs/day04.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| match u32::try_from(calc_wins_per_line(line)?) {
            Ok(0) => Some(0),
            Ok(n) => Some(2u32.pow(n) / 2),
            _ => None,
        })
        .sum::<u32>()
        .to_string()
}

fn part_b(input: &str) -> String {
    input
        .lines()
        .filter_map(calc_wins_per_line)
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

fn calc_wins_per_line(line: &str) -> Option<usize> {
    line.split_once(':').and_then(|(_, numbers)| {
        numbers.split_once('|').map(|(lhs, rhs)| {
            let winning_numbers = lhs
                .split_whitespace()
                .filter_map(|x| x.parse::<u8>().ok())
                .collect::<Vec<_>>();

            let out = rhs
                .split_whitespace()
                .filter_map(|x| x.parse::<u8>().ok())
                .filter(|x| winning_numbers.contains(x))
                .count();

            out
        })
    })
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

    #[test]
    fn result_a() {
        assert_eq!(part_a(INPUT), "23235");
    }

    #[test]
    fn result_b() {
        assert_eq!(part_b(INPUT), "5920640");
    }
}
