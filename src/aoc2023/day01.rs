const INPUT: &str = include_str!("inputs/day01.txt");
const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| {
            let a = line
                .matches(char::is_numeric)
                .next()
                .and_then(|s| str::parse::<u32>(s).ok());

            let b = line
                .matches(char::is_numeric)
                .last()
                .and_then(|s| str::parse::<u32>(s).ok());

            Some(10 * a? + b?)
        })
        .sum::<u32>()
        .to_string()
}

fn part_b(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| {
            let a = line.find(char::is_numeric).and_then(|n| {
                NUMBERS
                    .iter()
                    .filter_map(|s| line.find(s))
                    .min()
                    .filter(|&m| {
                        assert_ne!(m, n);
                        m < n
                    })
                    .map_or(
                        line.get(n..=n).and_then(|s| str::parse::<u32>(s).ok()),
                        |m| find_num_in_line_at_idx(line, m),
                    )
            });

            let b = line.rfind(char::is_numeric).and_then(|n| {
                NUMBERS
                    .iter()
                    .filter_map(|s| line.rfind(s))
                    .max()
                    .filter(|&m| {
                        assert_ne!(m, n);
                        m > n
                    })
                    .map_or(
                        line.get(n..=n).and_then(|s| str::parse::<u32>(s).ok()),
                        |m| find_num_in_line_at_idx(line, m),
                    )
            });

            Some(10 * a? + b?)
        })
        .sum::<u32>()
        .to_string()
}

fn find_num_in_line_at_idx(line: &str, idx: usize) -> Option<u32> {
    line.get(idx..=idx + 1).and_then(|s| match s {
        "on" => Some(1),
        "tw" => Some(2),
        "th" => Some(3),
        "fo" => Some(4),
        "fi" => Some(5),
        "si" => Some(6),
        "se" => Some(7),
        "ei" => Some(8),
        "ni" => Some(9),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST_A: &str = include_str!("tests/day01-a.txt");
    const _TEST_B: &str = include_str!("tests/day01-b.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST_A), "142");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST_B), "281");
    }

    #[test]
    fn result_a() {
        assert_eq!(part_a(INPUT), "54159");
    }

    #[test]
    fn result_b() {
        assert_eq!(part_b(INPUT), "53866");
    }
}
