use std::collections::HashMap;

const INPUT: &str = include_str!("inputs/day12.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(' ').unwrap();
            let nums: Vec<usize> = b.split(',').map(|num| num.parse().unwrap()).collect();

            possible_ways(&mut HashMap::new(), a.as_bytes(), None, &nums)
        })
        .sum::<usize>()
        .to_string()
}

fn part_b(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(' ').unwrap();
            let nums: Vec<usize> = b.split(',').map(|num| num.parse().unwrap()).collect();

            let vents: String = (a.to_string() + "?")
                .repeat(5)
                .chars()
                .take(5 * a.len() + 4)
                .collect();
            let new_nums = nums.repeat(5);

            possible_ways(&mut HashMap::new(), vents.as_bytes(), None, &new_nums)
        })
        .sum::<usize>()
        .to_string()
}

fn possible_ways(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    s: &[u8],
    within: Option<usize>,
    remaining: &[usize],
) -> usize {
    // Shamelessly taken from https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/12.rs
    if s.is_empty() {
        return match (within, remaining.len()) {
            (None, 0) => 1,
            (Some(x), 1) if x == remaining[0] => 1,
            _ => 0,
        };
    }
    if within.is_some() && remaining.is_empty() {
        return 0;
    }

    let key = (s.len(), within.unwrap_or(0), remaining.len());
    if let Some(&x) = cache.get(&key) {
        return x;
    }

    let ways = match (s[0], within) {
        (b'.', Some(x)) if x != remaining[0] => 0,
        (b'.', Some(_)) => possible_ways(cache, &s[1..], None, &remaining[1..]),
        (b'.', None) => possible_ways(cache, &s[1..], None, remaining),
        (b'#', Some(_)) => possible_ways(cache, &s[1..], within.map(|x| x + 1), remaining),
        (b'#', None) => possible_ways(cache, &s[1..], Some(1), remaining),
        (b'?', Some(x)) => {
            let mut ans = possible_ways(cache, &s[1..], within.map(|x| x + 1), remaining);
            if x == remaining[0] {
                ans += possible_ways(cache, &s[1..], None, &remaining[1..]);
            }
            ans
        }
        (b'?', None) => {
            possible_ways(cache, &s[1..], Some(1), remaining)
                + possible_ways(cache, &s[1..], None, remaining)
        }
        _ => unreachable!(),
    };
    cache.insert(key, ways);
    ways
}

// let p1 = possible_ways(Hashmap::new(), vents.as_bytes(), None, &nums);

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day12.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), "21");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), "525152");
    }
}
