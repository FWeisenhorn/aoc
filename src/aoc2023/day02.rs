use std::cmp::max;

const INPUT: &str = include_str!("inputs/day02.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| {
            line.split_once(':')
                .filter(|(_, games)| {
                    !games.split(&[',', ';']).any(|s| {
                        s[1..]
                            .split_once(char::is_whitespace)
                            .map_or(false, |(i, colour)| match colour {
                                "red" => i.parse::<u32>().is_ok_and(|x| x > 12),
                                "green" => i.parse::<u32>().is_ok_and(|x| x > 13),
                                "blue" => i.parse::<u32>().is_ok_and(|x| x > 14),
                                _ => unreachable!(),
                            })
                    })
                })
                .and_then(|(a, _)| {
                    a.split_whitespace()
                        .last()
                        .and_then(|s| s.parse::<u32>().ok())
                })
        })
        .sum::<u32>()
        .to_string()
}

pub fn part_b(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| {
            line.split_once(':').and_then(|(_, games)| {
                games
                    .split(&[',', ';'])
                    .try_fold([0; 3], |mut acc, game| {
                        game[1..]
                            .split_once(char::is_whitespace)
                            .and_then(|(i, colour)| {
                                i.parse::<u32>().map_or(None, |n| {
                                    match colour {
                                        "red" => acc[0] = max(acc[0], n),
                                        "blue" => acc[1] = max(acc[1], n),
                                        "green" => acc[2] = max(acc[2], n),
                                        _ => unreachable!(),
                                    }
                                    Some(acc)
                                })
                            })
                    })
                    .map(|arr| arr.into_iter().product::<u32>())
            })
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day02.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), "8");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), "2286");
    }
}
