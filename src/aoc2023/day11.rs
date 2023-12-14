use std::collections::HashSet;
type Pos = (usize, usize);

const INPUT: &str = include_str!("inputs/day11.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT, 1000000));
}

fn part_a(input: &str) -> String {
    part_b(input, 2)
}

fn part_b(input: &str, expansion: usize) -> String {
    let mut galaxies = HashSet::<Pos>::new();
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                galaxies.insert((i, j));
            };
        });
    });

    let max_x = input.lines().count();
    let max_y = input.lines().next().unwrap().chars().count();

    for x in (0..max_x).rev() {
        let empty_row = galaxies.iter().all(|(a, _)| a != &x);

        if empty_row {
            galaxies = galaxies
                .drain()
                .map(|(a, b)| {
                    if a > x {
                        (a + expansion - 1, b)
                    } else {
                        (a, b)
                    }
                })
                .collect();
        }
    }

    for y in (0..max_y).rev() {
        let empty_col = galaxies.iter().all(|(_, b)| b != &y);

        if empty_col {
            galaxies = galaxies
                .drain()
                .map(|(a, b)| {
                    if b > y {
                        (a, b + expansion - 1)
                    } else {
                        (a, b)
                    }
                })
                .collect();
        }
    }

    (galaxies.iter().fold(0, |acc, gal| {
        acc + galaxies
            .iter()
            .map(|other| gal.0.abs_diff(other.0) + gal.1.abs_diff(other.1))
            .sum::<usize>()
    }) / 2)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day11.txt");
    const _TESTRESULT_A: &str = "374";
    const _TESTRESULT_B1: &str = "1030";
    const _TESTRESULT_B2: &str = "8410";

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), _TESTRESULT_A);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST, 10), _TESTRESULT_B1);
        assert_eq!(part_b(_TEST, 100), _TESTRESULT_B2);
    }
}
