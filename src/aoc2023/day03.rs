use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("inputs/day03.txt");
const _TEST: &str = include_str!("tests/day03.txt");
const _TESTRESULT_A: &str = "4361";
const _TESTRESULT_B: &str = "467835";

type Pos = (usize, usize);

const NEIGHBOURS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut s: Vec<u32> = vec![];

    input.iter().enumerate().for_each(|(i, line)| {
        let mut cur: (u32, bool) = (0, false);

        line.iter().enumerate().for_each(|(j, c)| {
            if c.is_numeric() {
                // current char is part of a number
                cur.0 = cur.0 * 10 + c.to_digit(10).unwrap();

                // check if any neighbour is a punctuation mark (a)
                NEIGHBOURS
                    .iter()
                    .filter_map(|(a, b)| {
                        match (
                            (a + i32::try_from(i).unwrap()).try_into(),
                            (b + i32::try_from(j).unwrap()).try_into(),
                        ) {
                            (Ok(x), Ok(y)) => Some((x, y)),
                            _ => None,
                        }
                    })
                    .for_each(|(a, b): Pos| {
                        if let Some(line) = input.get(a) {
                            if let Some(c) = line.get(b) {
                                if c.is_ascii_punctuation() && *c != '.' {
                                    cur.1 = true;
                                }
                            }
                        }
                    });
            } else if cur.0 != 0 {
                // current char isn't part of a number
                if cur.1 {
                    s.push(cur.0);
                }
                cur = (0, false);
            }
        });
        // at end of line, store current number
        if cur.0 != 0 && cur.1 {
            s.push(cur.0);
        }
    });

    s.into_iter().sum::<u32>().to_string()
}

fn part_b(input: &str) -> String {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut t: HashMap<Pos, Vec<u32>> = HashMap::new();

    input.iter().enumerate().for_each(|(i, line)| {
        let mut cur: (u32, HashSet<Pos>) = (0, HashSet::new());

        line.iter().enumerate().for_each(|(j, c)| {
            if c.is_numeric() {
                // current char is part of a number
                cur.0 = cur.0 * 10 + c.to_digit(10).unwrap();

                // check if any neighbour is a punctuation mark (a) or '*' (b)
                NEIGHBOURS
                    .iter()
                    .filter_map(|(a, b)| {
                        match (
                            (a + i32::try_from(i).unwrap()).try_into(),
                            (b + i32::try_from(j).unwrap()).try_into(),
                        ) {
                            (Ok(x), Ok(y)) => Some((x, y)),
                            _ => None,
                        }
                    })
                    .for_each(|(a, b): Pos| {
                        if let Some(line) = input.get(a) {
                            if let Some(c) = line.get(b) {
                                if *c == '*' {
                                    cur.1.insert((a, b));
                                }
                            }
                        }
                    });
            } else if cur.0 != 0 {
                // current char isn't part of a number
                if !cur.1.is_empty() {
                    cur.1.iter().for_each(|p| {
                        t.entry(*p)
                            .and_modify(|x| x.push(cur.0))
                            .or_insert_with(|| vec![cur.0]);
                    });
                }
                cur = (0, HashSet::new());
            }
        });
        // at end of line, store current number
        if cur.0 != 0 && !cur.1.is_empty() {
            cur.1.iter().for_each(|p| {
                t.entry(*p)
                    .and_modify(|x| x.push(cur.0))
                    .or_insert_with(|| vec![cur.0]);
            });
        }
    });

    t.into_values()
        .filter_map(|v| {
            if v.len() == 2 {
                Some(v.iter().product::<u32>())
            } else {
                None
            }
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), _TESTRESULT_A);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), _TESTRESULT_B);
    }
}
