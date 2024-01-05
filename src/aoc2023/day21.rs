use crate::utils::{direction::NEIGHBOURS, pos::Pos};
use std::collections::HashSet;

const INPUT: &str = include_str!("inputs/day21.txt");

pub fn run() {
    println!("{}", part_a(INPUT, 64));
    // println!("{}", part_b(INPUT, 26_501_365),);
}

fn part_a(input: &str, n_steps: usize) -> String {
    let (data, start) = read_input(input);
    let mut possible_pos = HashSet::<Pos>::new();
    possible_pos.insert(start);

    let max_x = data.len();
    let max_y = data[0].len();

    for _ in 0..n_steps {
        let mut next_possible_pos = vec![];
        for pos in &possible_pos {
            NEIGHBOURS
                .iter()
                .filter_map(|&d| pos.steps_checked(d, 1, max_x, max_y))
                .for_each(|p| {
                    if data[p.x][p.y] == '.' {
                        next_possible_pos.push(p);
                    }
                });
        }

        possible_pos = next_possible_pos.into_iter().collect();
    }

    possible_pos.len().to_string()
}

// fn part_b(_input: &str, _n_steps: usize) -> String {
//     String::new()
// }

fn read_input(input: &str) -> (Vec<Vec<char>>, Pos) {
    let mut m: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let start = {
        let x_ = m.iter().position(|v| v.contains(&'S')).unwrap();
        let y_ = m[x_].iter().position(|&c| c == 'S').unwrap();

        Pos { x: x_, y: y_ }
    };

    m[start.x][start.y] = '.';

    (m, start)
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day21.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST, 6), "16");
    }

    // #[test]
    // fn test_b() {
    //     assert_eq!(part_b(_TEST, 10), "50");
    //     assert_eq!(part_b(_TEST, 50), "1594");
    //     assert_eq!(part_b(_TEST, 100), "6536");
    //     assert_eq!(part_b(_TEST, 500), "167004");
    //     assert_eq!(part_b(_TEST, 1000), "668697");
    //     assert_eq!(part_b(_TEST, 5000), "16733044");
    // }
}
