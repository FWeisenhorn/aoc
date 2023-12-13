const INPUT: &str = include_str!("inputs/day01.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            10 * line
                .matches(char::is_numeric)
                .next()
                .and_then(|s| str::parse::<u32>(s).ok())
                .unwrap()
                + line
                    .matches(char::is_numeric)
                    .last()
                    .and_then(|s| str::parse::<u32>(s).ok())
                    .unwrap()
        })
        .sum::<u32>()
        .to_string()
}

fn part_b(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let n_pos = line.find(char::is_numeric).unwrap();

            let m_pos = [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ]
            .iter()
            .filter_map(|s| line.find(s))
            .min();

            let a = match m_pos {
                Some(p) if p == n_pos => unreachable!(),
                Some(p) if p < n_pos => match line.get(p..=p + 1) {
                    Some("on") => 1,
                    Some("tw") => 2,
                    Some("th") => 3,
                    Some("fo") => 4,
                    Some("fi") => 5,
                    Some("si") => 6,
                    Some("se") => 7,
                    Some("ei") => 8,
                    Some("ni") => 9,
                    _ => unreachable!(),
                },
                _ => line
                    .get(n_pos..=n_pos)
                    .and_then(|s| str::parse::<u32>(s).ok())
                    .unwrap(),
            };

            let r_pos = line.rfind(char::is_numeric).unwrap();
            let s_pos = [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ]
            .iter()
            .filter_map(|s| line.rfind(s))
            .max();

            let b = match s_pos {
                Some(p) if p == r_pos => unreachable!(),
                Some(p) if p > r_pos => match line.get(p..=p + 1) {
                    Some("on") => 1,
                    Some("tw") => 2,
                    Some("th") => 3,
                    Some("fo") => 4,
                    Some("fi") => 5,
                    Some("si") => 6,
                    Some("se") => 7,
                    Some("ei") => 8,
                    Some("ni") => 9,
                    _ => unreachable!(),
                },
                _ => line
                    .get(r_pos..=r_pos)
                    .and_then(|s| str::parse::<u32>(s).ok())
                    .unwrap(),
            };
            a * 10 + b
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST_A: &str = include_str!("tests/day01-a.txt");
    const _TEST_B: &str = include_str!("tests/day01-b.txt");
    const _TESTRESULT_A: &str = "142";
    const _TESTRESULT_B: &str = "281";

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST_A), _TESTRESULT_A);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST_B), _TESTRESULT_B);
    }
}
