const INPUT: &str = include_str!("inputs/day05.txt");
const _TEST: &str = include_str!("tests/day05.txt");
const _TESTRESULT_A: &str = "35";
const _TESTRESULT_B: &str = "46";

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

// Part B is slow (~140s with --release flag)
//
// optimization ideas:
//  - Do some maths with the ranges that are being converted ?
//  - Optimize the conversions ?

fn part_a(input: &str) -> String {
    let input: Vec<&str> = input.lines().collect();

    let seeds: Vec<u64> = input[0]
        .split_at(7)
        .1
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let conv = read_conversion_tables(&input);

    seeds
        .iter()
        .map(|s| {
            conv.iter().fold(*s, |acc, x| {
                match x.iter().find(|[_, b, c]| *b <= acc && acc < *b + *c) {
                    Some([a, b, _]) => acc + a - b,
                    None => acc,
                }
            })
        })
        .min()
        .unwrap()
        .to_string()
}

fn part_b(input: &str) -> String {
    let input: Vec<&str> = input.lines().collect();

    let seeds = input[0]
        .split_at(7)
        .1
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();
    let seeds = seeds[..].chunks(2).map(|x| (x[0]..x[0] + x[1]));

    let conv = read_conversion_tables(&input);

    seeds
        .map(|r| {
            r.map(|s| {
                conv.iter().fold(s, |acc, x| {
                    match x.iter().find(|[_, b, c]| *b <= acc && acc < *b + *c) {
                        Some([a, b, _]) => acc + a - b,
                        None => acc,
                    }
                })
            })
            .min()
            .unwrap()
        })
        .min()
        .unwrap()
        .to_string()
}

fn read_conversion_tables(input: &[&str]) -> Vec<Vec<[u64; 3]>> {
    input.iter().skip(1).fold(vec![], |mut acc, line| {
        if line.starts_with(char::is_alphabetic) {
            acc.push(vec![]);
        }
        if line.starts_with(char::is_numeric) {
            let mut t = line.split_whitespace().map(|n| n.parse().unwrap());
            let t: [u64; 3] = [t.next().unwrap(), t.next().unwrap(), t.next().unwrap()];
            acc.last_mut().unwrap().push(t);
        }
        acc
    })
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
