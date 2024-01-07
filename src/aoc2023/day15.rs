const INPUT: &str = include_str!("inputs/day15.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(custom_hash_func)
                .map(u32::from)
                .sum::<u32>()
        })
        .sum::<u32>() // Because the input is only one line, this could also be .first().unwrap()
        .to_string()
}

fn part_b(input: &str) -> String {
    let mut boxes = vec![vec![]; 256];

    input.lines().for_each(|line| {
        line.split(',').for_each(|s| {
            if s.contains('=') {
                let (a, b) = s.split_once('=').unwrap();
                let pos = custom_hash_func(a) as usize;
                let focallength = b.parse::<usize>().unwrap();
                match boxes[pos].iter().position(|&(x, _)| x == a) {
                    Some(k) => {
                        boxes[pos][k].1 = focallength;
                    }
                    None => {
                        boxes[pos].push((a, focallength));
                    }
                }
            } else if s.contains('-') {
                let (a, _) = s.split_once('-').unwrap();
                let pos = custom_hash_func(a) as usize;

                if let Some(k) = boxes[pos].iter().position(|&(x, _)| x == a) {
                    boxes[pos].remove(k);
                }
            } else {
                unreachable!();
            }
        });
    });

    boxes
        .iter()
        .enumerate()
        .map(|(i, boxx)| {
            boxx.iter()
                .enumerate()
                .map(|(j, (_, focallength))| focallength * (i + 1) * (j + 1))
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

#[inline]
fn custom_hash_func(s: &str) -> u8 {
    s.chars()
        .fold(0u8, |acc, c| acc.wrapping_add(c as u8).wrapping_mul(17))
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day15.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), "1320");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), "145");
    }
}
