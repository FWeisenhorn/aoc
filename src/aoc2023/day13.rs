use crate::utils::pos::Pos;
use std::collections::HashSet;

const INPUT: &str = include_str!("inputs/day13.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    input
        .split("\n\n")
        .fold(0, |acc, s| {
            let rocks = get_rocks_from_input(s);

            let max_x: usize = rocks.iter().map(|p| p.x).max().unwrap();
            let max_y: usize = rocks.iter().map(|p| p.y).max().unwrap();

            for mirror_pos in 1..max_x {
                if rocks
                    .iter()
                    .filter(|rock| {
                        (rock.x <= mirror_pos && 2 * mirror_pos < max_x + rock.x)
                            || (rock.x > mirror_pos && 2 * mirror_pos >= rock.x)
                    })
                    .all(|rock| {
                        rocks.contains(&Pos {
                            x: 2 * mirror_pos + 1 - rock.x,
                            y: rock.y,
                        })
                    })
                {
                    return acc + mirror_pos * 100;
                }
            }

            for mirror_pos in 1..max_y {
                if rocks
                    .iter()
                    .filter(|rock| {
                        (rock.y <= mirror_pos && 2 * mirror_pos < max_y + rock.y)
                            || (rock.y > mirror_pos && 2 * mirror_pos >= rock.y)
                    })
                    .all(|rock| {
                        rocks.contains(&Pos {
                            x: rock.x,
                            y: 2 * mirror_pos + 1 - rock.y,
                        })
                    })
                {
                    return acc + mirror_pos;
                }
            }

            unreachable!()
        })
        .to_string()
}

fn part_b(input: &str) -> String {
    input
        .split("\n\n")
        .fold(0, |acc, s| {
            let rocks = get_rocks_from_input(s);

            let max_x: usize = rocks.iter().map(|p| p.x).max().unwrap();
            let max_y: usize = rocks.iter().map(|p| p.y).max().unwrap();

            for mirror_pos in 1..max_x {
                if rocks
                    .iter()
                    .filter(|rock| {
                        (rock.x <= mirror_pos && 2 * mirror_pos < max_x + rock.x)
                            || (rock.x > mirror_pos && 2 * mirror_pos >= rock.x)
                    })
                    .filter(|rock| {
                        !rocks.contains(&Pos {
                            x: 2 * mirror_pos + 1 - rock.x,
                            y: rock.y,
                        })
                    })
                    .count()
                    == 1
                {
                    return acc + mirror_pos * 100;
                }
            }

            for mirror_pos in 1..max_y {
                if rocks
                    .iter()
                    .filter(|rock| {
                        (rock.y <= mirror_pos && 2 * mirror_pos < max_y + rock.y)
                            || (rock.y > mirror_pos && 2 * mirror_pos >= rock.y)
                    })
                    .filter(|rock| {
                        !rocks.contains(&Pos {
                            x: rock.x,
                            y: 2 * mirror_pos + 1 - rock.y,
                        })
                    })
                    .count()
                    == 1
                {
                    return acc + mirror_pos;
                }
            }

            unreachable!()
        })
        .to_string()
}

fn get_rocks_from_input(input: &str) -> HashSet<Pos> {
    let mut rocks = HashSet::<Pos>::new();
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                rocks.insert(Pos { x: i + 1, y: j + 1 });
            };
        });
    });
    rocks
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day13.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), "405");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), "400");
    }
}
