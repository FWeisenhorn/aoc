use std::collections::HashSet;
const INPUT: &str = include_str!("inputs/day21.txt");

type Pos = (usize, usize);

pub fn run() {
    println!("{}", part_a(INPUT, 64));
    println!("{}", part_b(INPUT, 26_501_365),);
}

fn part_a(input: &str, n_steps: usize) -> String {
    let (data, start) = read_input(input);
    let mut possible_pos = HashSet::<Pos>::new();
    possible_pos.insert(start);

    for _ in 0..n_steps {
        let mut next_possible_pos = HashSet::<Pos>::new();
        for pos in &possible_pos {
            for step in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_pos = (pos.0 as i32 + step.0, pos.1 as i32 + step.1);

                if 0 <= new_pos.0
                    && (new_pos.0 as usize) < data.len()
                    && 0 <= new_pos.1
                    && (new_pos.1 as usize) < data[0].len()
                {
                    let (x, y) = (new_pos.0 as usize, new_pos.1 as usize);
                    if data[x][y] == '.' {
                        next_possible_pos.insert((x, y));
                    }
                }
            }
        }

        possible_pos = next_possible_pos;
    }

    possible_pos.len().to_string()
}

fn part_b(input: &str, n_steps: usize) -> String {
    String::new()
}

fn read_input(input: &str) -> (Vec<Vec<char>>, Pos) {
    let mut k: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let x = k.iter().position(|v| v.contains(&'S')).unwrap();

    let y = k[x].iter().position(|&c| c == 'S').unwrap();

    k[x][y] = '.';

    (k, (x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day21.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST, 6), "16");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST, 10), "50");
        assert_eq!(part_b(_TEST, 50), "1594");
        assert_eq!(part_b(_TEST, 100), "6536");
        assert_eq!(part_b(_TEST, 500), "167004");
        assert_eq!(part_b(_TEST, 1000), "668697");
        assert_eq!(part_b(_TEST, 5000), "16733044");
    }
}
