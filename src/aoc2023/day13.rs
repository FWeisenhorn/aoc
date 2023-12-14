use std::collections::HashSet;

type Pos = (usize, usize);

const INPUT: &str = include_str!("inputs/day13.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    input
        .split("\n\n")
        .map(|s| {
            let rocks = get_rocks_from_input(s);

            let max_x: usize = rocks.iter().max_by_key(|(x, _)| x).unwrap().0;
            let max_y: usize = rocks.iter().max_by_key(|(_, x)| x).unwrap().1;
            let mut i = 0;

            for mirror_pos in 1..max_x {
                if rocks
                    .iter()
                    .filter(|rock| {
                        (rock.0 <= mirror_pos && 2 * mirror_pos < max_x + rock.0)
                            || (rock.0 > mirror_pos && 2 * mirror_pos >= rock.0)
                    })
                    .all(|rock| rocks.contains(&(2 * mirror_pos + 1 - rock.0, rock.1)))
                {
                    i = mirror_pos * 100;
                }
            }

            for mirror_pos in 1..max_y {
                if rocks
                    .iter()
                    .filter(|rock| {
                        (rock.1 <= mirror_pos && 2 * mirror_pos < max_y + rock.1)
                            || (rock.1 > mirror_pos && 2 * mirror_pos >= rock.1)
                    })
                    .all(|rock| rocks.contains(&(rock.0, 2 * mirror_pos + 1 - rock.1)))
                {
                    i = mirror_pos;
                }
            }

            i
        })
        .sum::<usize>()
        .to_string()
}

fn part_b(input: &str) -> String {
    input
        .split("\n\n")
        .map(|s| {
            let rocks = get_rocks_from_input(s);

            let max_x: usize = rocks.iter().max_by_key(|(x, _)| x).unwrap().0;
            let max_y: usize = rocks.iter().max_by_key(|(_, x)| x).unwrap().1;
            let mut i = 0;

            for mirror_pos in 1..max_x {
                if rocks
                    .iter()
                    .filter(|rock| {
                        (rock.0 <= mirror_pos && 2 * mirror_pos < max_x + rock.0)
                            || (rock.0 > mirror_pos && 2 * mirror_pos >= rock.0)
                    })
                    .filter(|rock| !rocks.contains(&(2 * mirror_pos + 1 - rock.0, rock.1)))
                    .count()
                    == 1
                {
                    i = mirror_pos * 100;
                }
            }

            for mirror_pos in 1..max_y {
                if rocks
                    .iter()
                    .filter(|rock| {
                        (rock.1 <= mirror_pos && 2 * mirror_pos < max_y + rock.1)
                            || (rock.1 > mirror_pos && 2 * mirror_pos >= rock.1)
                    })
                    .filter(|rock| !rocks.contains(&(rock.0, 2 * mirror_pos + 1 - rock.1)))
                    .count()
                    == 1
                {
                    i = mirror_pos;
                }
            }

            i
        })
        .sum::<usize>()
        .to_string()
}

fn get_rocks_from_input(input: &str) -> HashSet<Pos> {
    let mut rocks = HashSet::<Pos>::new();
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                rocks.insert((i + 1, j + 1));
            };
        });
    });
    rocks
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day13.txt");
    const _TESTRESULT_A: &str = "405";
    const _TESTRESULT_B: &str = "400";

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), _TESTRESULT_A);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), _TESTRESULT_B);
    }
}
