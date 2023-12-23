use std::collections::HashMap;

const INPUT: &str = include_str!("inputs/day14.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    let mut data: Vec<char> = input.lines().flat_map(str::chars).collect();
    let height = input.lines().count();

    tilt_north(&mut data, height);

    get_answer(&data, height)
}

fn part_b(input: &str) -> String {
    let mut data: Vec<char> = input.lines().flat_map(str::chars).collect();
    let height = input.lines().count();

    let spin_cycles = 1_000_000_000u32;

    let mut seen: HashMap<Vec<char>, u32> = HashMap::new();

    for i in 0..spin_cycles {
        if let Some(seen_at) = seen.insert(data.clone(), i) {
            if (spin_cycles - i) % (i - seen_at) == 0 {
                break;
            }
        }

        tilt_north(&mut data, height);
        tilt_west(&mut data, height);
        tilt_south(&mut data, height);
        tilt_east(&mut data, height);
    }

    get_answer(&data, height)
}

fn get_answer(data: &Vec<char>, height: usize) -> String {
    let width = data.len() / height;
    data.iter()
        .enumerate()
        .filter(|(_, &c)| c == 'O')
        .fold(0usize, |acc, (i, _)| acc + (height - i / width))
        .to_string()
}

fn tilt_north(data: &mut Vec<char>, height: usize) {
    let width = data.len() / height;
    let mut has_changed = true;
    while has_changed {
        has_changed = false;
        for x in 0..height - 1 {
            for y in 0..width {
                let coord = x * width + y;
                if data[coord] == '.' && data[coord + width] == 'O' {
                    data[coord] = 'O';
                    data[coord + width] = '.';
                    has_changed = true;
                }
            }
        }
    }
}

fn tilt_west(data: &mut Vec<char>, height: usize) {
    let width = data.len() / height;
    let mut has_changed = true;
    while has_changed {
        has_changed = false;
        for x in 0..height {
            for y in 0..width - 1 {
                let coord = x * width + y;
                if data[coord] == '.' && data[coord + 1] == 'O' {
                    data[coord] = 'O';
                    data[coord + 1] = '.';
                    has_changed = true;
                }
            }
        }
    }
}

fn tilt_south(data: &mut Vec<char>, height: usize) {
    let width = data.len() / height;
    let mut has_changed = true;
    while has_changed {
        has_changed = false;
        for x in (1..height).rev() {
            for y in (0..width).rev() {
                let coord = x * width + y;
                if data[coord] == '.' && data[coord - width] == 'O' {
                    data[coord] = 'O';
                    data[coord - width] = '.';
                    has_changed = true;
                }
            }
        }
    }
}

fn tilt_east(data: &mut Vec<char>, height: usize) {
    let width = data.len() / height;
    let mut has_changed = true;
    while has_changed {
        has_changed = false;
        for x in (0..height).rev() {
            for y in (1..width).rev() {
                let coord = x * width + y;
                if data[coord] == '.' && data[coord - 1] == 'O' {
                    data[coord] = 'O';
                    data[coord - 1] = '.';
                    has_changed = true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day14.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), "136");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), "64");
    }
}
