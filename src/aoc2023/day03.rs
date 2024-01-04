use crate::utils::{direction::Direction, pos::Pos};
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("inputs/day03.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut s: Vec<u32> = vec![];

    data.iter().enumerate().for_each(|(i, line)| {
        let mut cur: (u32, bool) = (0, false);

        line.iter().enumerate().for_each(|(j, c)| {
            if c.is_numeric() {
                // current char is part of a number
                cur.0 = cur.0 * 10 + c.to_digit(10).unwrap();

                // check if any neighbour is a punctuation mark (a)
                neighbours()
                    .iter()
                    .filter_map(|v| {
                        v.iter().try_fold(Pos { x: i, y: j }, |acc, &dir| {
                            acc.steps_checked(dir, 1, data.len(), data[0].len())
                        })
                    })
                    .for_each(|p| {
                        if let Some(line) = data.get(p.x) {
                            if let Some(c) = line.get(p.y) {
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
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut t: HashMap<Pos, Vec<u32>> = HashMap::new();

    data.iter().enumerate().for_each(|(i, line)| {
        let mut cur: (u32, HashSet<Pos>) = (0, HashSet::new());

        line.iter().enumerate().for_each(|(j, c)| {
            if c.is_numeric() {
                // current char is part of a number
                cur.0 = cur.0 * 10 + c.to_digit(10).unwrap();

                // check if any neighbour is a punctuation mark (a) or '*' (b)
                neighbours()
                    .iter()
                    .filter_map(|v| {
                        v.iter().try_fold(Pos { x: i, y: j }, |acc, &dir| {
                            acc.steps_checked(dir, 1, data.len(), data[0].len())
                        })
                    })
                    .for_each(|p| {
                        if let Some(line) = data.get(p.x) {
                            if let Some(c) = line.get(p.y) {
                                if *c == '*' {
                                    cur.1.insert(p);
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

fn neighbours() -> [Vec<Direction>; 8] {
    [
        vec![Direction::Up, Direction::Left],
        vec![Direction::Up],
        vec![Direction::Up, Direction::Right],
        vec![Direction::Left],
        vec![Direction::Right],
        vec![Direction::Down, Direction::Left],
        vec![Direction::Down],
        vec![Direction::Down, Direction::Right],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day03.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), "4361");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), "467835");
    }
}
