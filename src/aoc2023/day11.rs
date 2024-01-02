use crate::utils::pos::Pos;

const INPUT: &str = include_str!("inputs/day11.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT, 1_000_000));
}

fn part_a(input: &str) -> String {
    part_b(input, 2)
}

fn part_b(input: &str, expansion: usize) -> String {
    let mut galaxies: Vec<Pos> = vec![];
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                galaxies.push(Pos { x: i, y: j });
            };
        });
    });

    let max_x = input.lines().count();
    let max_y = input.lines().next().unwrap().chars().count();

    for x_ in (0..max_x).rev() {
        if !galaxies.iter().any(|&p| p.x == x_) {
            galaxies.iter_mut().filter(|p| p.x > x_).for_each(|p| {
                p.x += expansion - 1;
            });
        }
    }

    for y_ in (0..max_y).rev() {
        if !galaxies.iter().any(|&p| p.y == y_) {
            galaxies.iter_mut().filter(|p| p.y > y_).for_each(|p| {
                p.y += expansion - 1;
            });
        }
    }

    (galaxies
        .iter()
        .map(|gal| {
            galaxies
                .iter()
                .map(|other| gal.distance(other))
                .sum::<usize>()
        })
        .sum::<usize>()
        / 2)
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day11.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), "374");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST, 10), "1030");
        assert_eq!(part_b(_TEST, 100), "8410");
    }
}
