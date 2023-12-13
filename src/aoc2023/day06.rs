const INPUT: &str = include_str!("inputs/day06.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    let input: Vec<&str> = input.lines().collect();

    let times = input[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u32>().unwrap());
    let dists = input[1]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u32>().unwrap());

    times
        .zip(dists)
        .map(|(t, d)| (0..t).filter(|i| (t - i) * i > d).count())
        .product::<usize>()
        .to_string()
}

fn part_b(input: &str) -> String {
    let input: Vec<&str> = input.lines().collect();

    let time: u64 = input[0]
        .split_whitespace()
        .skip(1)
        .flat_map(str::chars)
        .collect::<String>()
        .parse()
        .unwrap();

    let dist: u64 = input[1]
        .split_whitespace()
        .skip(1)
        .flat_map(str::chars)
        .collect::<String>()
        .parse()
        .unwrap();

    (0..time)
        .filter(|i| (time - i) * i > dist)
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day06.txt");
    const _TESTRESULT_A: &str = "288";
    const _TESTRESULT_B: &str = "71503";

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), _TESTRESULT_A);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), _TESTRESULT_B);
    }
}
