use std::collections::HashMap;

const INPUT: &str = include_str!("inputs/day08.txt");
const _TEST_A: &str = include_str!("tests/day08-a.txt");
const _TEST_B: &str = include_str!("tests/day08-b.txt");
const _TESTRESULT_A: &str = "6";
const _TESTRESULT_B: &str = "6";

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    let input = input.split_once("\n\n").unwrap();

    let mut dirs = input.0.chars().cycle();

    let choices_map: HashMap<&str, [&str; 2]> = input
        .1
        .lines()
        .map(|s| (&s[0..3], [&s[7..10], &s[12..15]]))
        .collect::<HashMap<&str, [&str; 2]>>();

    let mut i = 0u64;
    let mut cur_point = "AAA";

    loop {
        if cur_point == "ZZZ" {
            break;
        }

        i += 1;

        cur_point = match dirs.next() {
            Some('L') => choices_map[cur_point][0],
            Some('R') => choices_map[cur_point][1],
            _ => unreachable!(),
        }
    }
    i.to_string()
}

fn part_b(input: &str) -> String {
    let input = input.split_once("\n\n").unwrap();

    let dirs = input.0.chars().cycle();

    let choices_map: HashMap<&str, [&str; 2]> = input
        .1
        .lines()
        .map(|s| (&s[0..3], [&s[7..10], &s[12..15]]))
        .collect::<HashMap<&str, [&str; 2]>>();

    let starting_points: Vec<&str> = choices_map
        .keys()
        .filter(|s| s.ends_with('A'))
        .copied()
        .collect();

    starting_points
        .iter()
        .map(|starting_point| {
            let mut i = 0u64;
            let mut d = dirs.clone();
            let mut cur_point = *starting_point;

            loop {
                if cur_point.ends_with('Z') {
                    break;
                }

                i += 1;

                cur_point = match d.next() {
                    Some('L') => choices_map[cur_point][0],
                    Some('R') => choices_map[cur_point][1],
                    _ => unreachable!(),
                };
            }
            i
        })
        .fold(1u64, lcm)
        .to_string()
}

#[inline]
const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

#[inline]
const fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST_A), _TESTRESULT_A);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST_B), _TESTRESULT_B);
    }
}
