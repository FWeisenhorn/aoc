const INPUT: &str = include_str!("inputs/day09.txt");
const _TEST: &str = include_str!("tests/day09.txt");
const _TESTRESULT_A: &str = "114";
const _TESTRESULT_B: &str = "2";

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let values: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            values.last().unwrap() + calc_next(&vec_diff(&values))
        })
        .sum::<i32>()
        .to_string()
}

fn part_b(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let values: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            values.first().unwrap() - calc_prev(&vec_diff(&values))
        })
        .sum::<i32>()
        .to_string()
}

fn vec_diff(v: &[i32]) -> Vec<i32> {
    let vals = v.iter();
    let next_vals = v.iter().skip(1);

    vals.zip(next_vals).map(|(cur, next)| next - cur).collect()
}

fn calc_next(v: &[i32]) -> i32 {
    if vec_diff(v).iter().all(|x| *x == 0) {
        *v.last().unwrap()
    } else {
        *v.last().unwrap() + calc_next(&vec_diff(v))
    }
}

fn calc_prev(v: &[i32]) -> i32 {
    if vec_diff(v).iter().all(|x| *x == 0) {
        *v.first().unwrap()
    } else {
        *v.first().unwrap() - calc_prev(&vec_diff(v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), _TESTRESULT_A);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), _TESTRESULT_B);
    }
}
