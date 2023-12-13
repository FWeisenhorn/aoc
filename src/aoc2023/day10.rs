use std::collections::HashMap;

const INPUT: &str = include_str!("inputs/day10.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

type Pos = (usize, usize);

fn part_a(input: &str) -> String {
    let m: HashMap<Pos, char> = input
        .lines()
        .enumerate()
        .flat_map(|(i, x)| x.chars().enumerate().map(move |(j, c)| ((i, j), c)))
        .collect();

    let start = m.iter().find(|(_, &v)| v == 'S').unwrap().0;

    let mut i: u32 = 0;
    let (mut x, mut y) = start;
    let mut dir: char = '0';

    // 1. find initial direction
    if let Some('|' | 'J' | 'L') = m.get(&(x + 1, y)) {
        dir = 'd';
    }

    if let Some('|' | 'F' | '7') = m.get(&(x - 1, y)) {
        dir = 'u';
    }

    if let Some('-' | 'J' | '7') = m.get(&(x, y + 1)) {
        dir = 'r';
    };

    assert_ne!(dir, '0');

    // 2. go through the loop until we are back at the start
    loop {
        i += 1;

        match dir {
            'u' => {
                (x, y) = (x - 1, y);
                match m.get(&(x, y)) {
                    Some('S') => {
                        break;
                    }
                    Some('|') => (),
                    Some('7') => {
                        dir = 'l';
                    }
                    Some('F') => {
                        dir = 'r';
                    }
                    _ => unreachable!(),
                }
            }
            'd' => {
                (x, y) = (x + 1, y);
                match m.get(&(x, y)) {
                    Some('S') => {
                        break;
                    }
                    Some('|') => (),
                    Some('J') => {
                        dir = 'l';
                    }
                    Some('L') => {
                        dir = 'r';
                    }
                    _ => unreachable!(),
                }
            }
            'l' => {
                (x, y) = (x, y - 1);
                match m.get(&(x, y)) {
                    Some('S') => {
                        break;
                    }
                    Some('-') => (),
                    Some('L') => {
                        dir = 'u';
                    }
                    Some('F') => {
                        dir = 'd';
                    }
                    _ => unreachable!(),
                }
            }
            'r' => {
                (x, y) = (x, y + 1);
                match m.get(&(x, y)) {
                    Some('S') => {
                        break;
                    }
                    Some('-') => (),
                    Some('J') => {
                        dir = 'u';
                    }
                    Some('7') => {
                        dir = 'd';
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }

    (i / 2).to_string()
}

fn part_b(input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    const _TEST_A1: &str = include_str!("tests/day10-a1.txt");
    const _TEST_A2: &str = include_str!("tests/day10-a2.txt");
    const _TESTRESULT_A1: &str = "4";
    const _TESTRESULT_A2: &str = "8";
    const _TEST_B1: &str = include_str!("tests/day10-b1.txt");
    const _TEST_B2: &str = include_str!("tests/day10-b2.txt");
    const _TEST_B3: &str = include_str!("tests/day10-b3.txt");
    const _TESTRESULT_B1: &str = "4";
    const _TESTRESULT_B2: &str = "8";
    const _TESTRESULT_B3: &str = "10";

    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST_A1), _TESTRESULT_A1);
        assert_eq!(part_a(_TEST_A2), _TESTRESULT_A2);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST_B1), _TESTRESULT_B1);
        assert_eq!(part_b(_TEST_B2), _TESTRESULT_B2);
        assert_eq!(part_b(_TEST_B3), _TESTRESULT_B3);
    }
}
