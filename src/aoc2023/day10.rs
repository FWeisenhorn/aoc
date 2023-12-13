use std::collections::{HashMap, HashSet, VecDeque};

type Pos = (usize, usize);
type SignedPos = (i32, i32);

const INPUT: &str = include_str!("inputs/day10.txt");
const NEIGHBOURS: [SignedPos; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

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
    let input: HashMap<SignedPos, char> = input
        .lines()
        .enumerate()
        .flat_map(|(i, x)| {
            x.chars()
                .enumerate()
                .map(move |(j, c)| ((i.try_into().unwrap(), j.try_into().unwrap()), c))
        })
        .collect();

    let mut visited = HashSet::<SignedPos>::new();
    let mut passed_rhs = HashSet::<SignedPos>::new();
    let mut passed_lhs = HashSet::<SignedPos>::new();

    let start = input.iter().find(|(_, &v)| v == 'S').unwrap().0;

    let mut i = 0; // will be > 0 if the route was walked in a right turn

    let (mut x, mut y) = start;
    let mut dir: char = '0';

    if let Some('|' | 'J' | 'L') = input.get(&(x + 1, y)) {
        dir = 'd';
    }

    if let Some('|' | 'F' | '7') = input.get(&(x - 1, y)) {
        dir = 'u';
    }

    if let Some('-' | 'J' | '7') = input.get(&(x, y + 1)) {
        dir = 'r';
    };

    assert_ne!(dir, '0');

    loop {
        visited.insert((x, y));

        match dir {
            'u' => {
                (x, y) = (x - 1, y);
                match input.get(&(x, y)) {
                    Some('S') => {
                        break;
                    }
                    Some('|') => {
                        if input.contains_key(&(x, y - 1)) {
                            passed_lhs.insert((x, y - 1));
                        }
                        if input.contains_key(&(x, y + 1)) {
                            passed_rhs.insert((x, y + 1));
                        }
                    }
                    Some('7') => {
                        dir = 'l';
                        i -= 1;
                        if input.contains_key(&(x - 1, y)) {
                            passed_rhs.insert((x - 1, y));
                        }
                        if input.contains_key(&(x, y + 1)) {
                            passed_rhs.insert((x, y + 1));
                        }
                    }
                    Some('F') => {
                        dir = 'r';
                        i += 1;
                        if input.contains_key(&(x - 1, y)) {
                            passed_lhs.insert((x - 1, y));
                        }
                        if input.contains_key(&(x, y - 1)) {
                            passed_lhs.insert((x, y - 1));
                        }
                    }
                    _ => unreachable!(),
                }
            }
            'd' => {
                (x, y) = (x + 1, y);
                match input.get(&(x, y)) {
                    Some('S') => {
                        break;
                    }
                    Some('|') => {
                        if input.contains_key(&(x, y - 1)) {
                            passed_rhs.insert((x, y - 1));
                        }
                        if input.contains_key(&(x, y + 1)) {
                            passed_lhs.insert((x, y + 1));
                        }
                    }
                    Some('J') => {
                        dir = 'l';
                        i += 1;
                        if input.contains_key(&(x + 1, y)) {
                            passed_lhs.insert((x + 1, y));
                        }
                        if input.contains_key(&(x, y + 1)) {
                            passed_lhs.insert((x, y + 1));
                        }
                    }
                    Some('L') => {
                        dir = 'r';
                        i -= 1;
                        if input.contains_key(&(x + 1, y)) {
                            passed_rhs.insert((x + 1, y));
                        }
                        if input.contains_key(&(x, y - 1)) {
                            passed_rhs.insert((x, y - 1));
                        }
                    }
                    _ => unreachable!(),
                }
            }
            'l' => {
                (x, y) = (x, y - 1);
                match input.get(&(x, y)) {
                    Some('S') => {
                        break;
                    }
                    Some('-') => {
                        if input.contains_key(&(x - 1, y)) {
                            passed_rhs.insert((x - 1, y));
                        }
                        if input.contains_key(&(x + 1, y)) {
                            passed_lhs.insert((x + 1, y));
                        }
                    }
                    Some('L') => {
                        dir = 'u';
                        i += 1;
                        if input.contains_key(&(x + 1, y)) {
                            passed_lhs.insert((x + 1, y));
                        }
                        if input.contains_key(&(x, y - 1)) {
                            passed_lhs.insert((x, y - 1));
                        }
                    }
                    Some('F') => {
                        dir = 'd';
                        i -= 1;
                        if input.contains_key(&(x - 1, y)) {
                            passed_rhs.insert((x - 1, y));
                        }
                        if input.contains_key(&(x, y - 1)) {
                            passed_rhs.insert((x, y - 1));
                        }
                    }
                    _ => unreachable!(),
                }
            }
            'r' => {
                (x, y) = (x, y + 1);
                match input.get(&(x, y)) {
                    Some('S') => {
                        break;
                    }
                    Some('-') => {
                        if input.contains_key(&(x - 1, y)) {
                            passed_lhs.insert((x - 1, y));
                        }
                        if input.contains_key(&(x + 1, y)) {
                            passed_rhs.insert((x + 1, y));
                        }
                    }
                    Some('J') => {
                        dir = 'u';
                        i -= 1;
                        if input.contains_key(&(x + 1, y)) {
                            passed_rhs.insert((x + 1, y));
                        }
                        if input.contains_key(&(x, y + 1)) {
                            passed_rhs.insert((x, y + 1));
                        }
                    }
                    Some('7') => {
                        dir = 'd';
                        i += 1;
                        if input.contains_key(&(x - 1, y)) {
                            passed_lhs.insert((x - 1, y));
                        }
                        if input.contains_key(&(x, y + 1)) {
                            passed_lhs.insert((x, y + 1));
                        }
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }

    let mut t = if i > 0 { passed_rhs } else { passed_lhs };
    t.retain(|p| !visited.contains(p));

    expand_queue(t, &input.into_keys().collect(), &visited)
        .len()
        .to_string()
}

fn expand_queue(
    to_expand: HashSet<SignedPos>,
    base_region: &HashSet<SignedPos>,
    visited: &HashSet<SignedPos>,
) -> HashSet<SignedPos> {
    let mut to_expand: VecDeque<SignedPos> = to_expand.into_iter().collect();
    let mut expanded = HashSet::<SignedPos>::new();

    while !to_expand.is_empty() {
        let cur = to_expand.pop_front().unwrap();
        NEIGHBOURS
            .iter()
            .map(|nei| (nei.0 + cur.0, nei.1 + cur.1))
            .for_each(|p| {
                if base_region.contains(&p)
                    && !(expanded.contains(&p) || to_expand.contains(&p) || visited.contains(&p))
                {
                    to_expand.push_back(p);
                }
            });

        expanded.insert(cur);
    }

    expanded
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
