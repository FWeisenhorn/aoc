use std::cmp;

const INPUT: &str = include_str!("inputs/day05.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    let data: Vec<&str> = input.lines().collect();

    let conv = read_conversion_tables(&data);

    data[0][7..]
        .split_whitespace()
        .filter_map(|x| x.parse::<u64>().ok())
        .map(|s| {
            conv.iter().fold(s, |acc, x| {
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

    let seeds: Vec<u64> = input[0][7..]
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    let mut seeds: Vec<[u64; 2]> = seeds[..]
        .chunks(2)
        .map(|x| [x[0], x[0] + x[1] - 1])
        .collect();

    let conv = read_conversion_tables(&input);

    for c in conv {
        seeds = convert_ranges(&seeds, &c);
    }

    seeds.into_iter().map(|r| r[0]).min().unwrap().to_string()
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

fn convert_ranges(v: &[[u64; 2]], convert_table: &[[u64; 3]]) -> Vec<[u64; 2]> {
    // v: {[start, end]} (inclusive)
    // c: {[dest_begin, source_begin, len]} (exclusive at end)
    let mut to_process: Vec<_> = v.into();
    to_process.sort_by_key(|t| t[0]);

    let mut out = vec![];

    for c in convert_table {
        let mut still_to_process = vec![];
        for r in to_process {
            if let Some(t) = convertible_subrange(&r, c) {
                if r[0] < t[0] {
                    still_to_process.push([r[0], t[0] - 1]);
                }
                if t[1] < r[1] {
                    still_to_process.push([t[1] + 1, r[1]]);
                }
                out.push([t[0] + c[0] - c[1], t[1] + c[0] - c[1]]);
            } else {
                still_to_process.push(r);
            }
        }

        still_to_process.sort_by_key(|t| t[0]);
        to_process = still_to_process;
    }

    out.append(&mut to_process);
    out.sort_by_key(|t| t[0]);
    out
}

fn convertible_subrange(r: &[u64; 2], c: &[u64; 3]) -> Option<[u64; 2]> {
    let t1 = cmp::max(r[0], c[1]);
    let t2 = cmp::min(r[1], c[1] + c[2] - 1);

    if t1 <= t2 {
        Some([t1, t2])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day05.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), "35");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), "46");
    }

    #[test]
    fn result_a() {
        assert_eq!(part_a(INPUT), "600279879");
    }

    #[test]
    fn result_b() {
        assert_eq!(part_b(INPUT), "20191102");
    }
}
