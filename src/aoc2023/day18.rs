use crate::utils::direction::Direction;

const INPUT: &str = include_str!("inputs/day18.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    let trench_instr: Vec<(Direction, u32)> = input
        .lines()
        .filter_map(|line| {
            let t: Vec<&str> = line.splitn(3, char::is_whitespace).collect();

            Direction::try_from(t[0].chars().next()?)
                .ok()
                .zip(t[1].parse::<u32>().ok())
        })
        .collect();

    let trenches = trench_corners(&trench_instr);

    calc_dig_area(&trenches).to_string()
}

fn part_b(input: &str) -> String {
    let trench_instr: Vec<(Direction, u32)> = input
        .lines()
        .filter_map(|line| {
            let (_, t) = line.split_once('#')?;

            let dir = match &t[t.len() - 2..t.len() - 1] {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => unreachable!(),
            };

            let n = u32::from_str_radix(&t[0..t.len() - 2], 16).ok();

            Some((dir, n?))
        })
        .collect();

    let trenches = trench_corners(&trench_instr);

    calc_dig_area(&trenches).to_string()
}

fn trench_corners(trench_instr: &[(Direction, u32)]) -> Vec<(i64, i64)> {
    let mut out = vec![];
    let p = trench_instr.iter().fold((0i64, 0i64), |acc, &(d, n)| {
        out.push(acc);
        match d {
            Direction::Up => (acc.0 - i64::from(n), acc.1),
            Direction::Down => (acc.0 + i64::from(n), acc.1),
            Direction::Left => (acc.0, acc.1 - i64::from(n)),
            Direction::Right => (acc.0, acc.1 + i64::from(n)),
        }
    });
    assert_eq!(p, (0, 0));

    out.push(p);
    out
}

fn calc_dig_area(trench_corners: &[(i64, i64)]) -> i64 {
    // let n = trench_corners.len();

    let (a, b) = trench_corners.windows(2).fold((0, 0), |acc, p| {
        let t = (p[0].0 * p[1].1).wrapping_sub(p[0].1 * p[1].0);
        let d = i64::try_from(p[0].0.abs_diff(p[1].0) + p[0].1.abs_diff(p[1].1)).unwrap();

        (acc.0 + t, acc.1 + d)
    });

    (a.abs() + b) / 2 + 1
}
#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day18.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), "62");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), "952408144115");
    }
}
