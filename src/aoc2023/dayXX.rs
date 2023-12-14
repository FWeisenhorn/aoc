const INPUT: &str = include_str!("inputs/dayXX.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    String::new()
}

fn part_b(input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/dayXX.txt");
    const _TESTRESULT_A: &str = "";
    const _TESTRESULT_B: &str = "";

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), _TESTRESULT_A);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), _TESTRESULT_B);
    }
}