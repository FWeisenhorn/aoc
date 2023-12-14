type Pos = (usize, usize);

const INPUT: &str = include_str!("inputs/day14.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    let stable_rocks = get_rocks_from_input(input, '#');
    let mut movable_rocks = get_rocks_from_input(input, 'O');

    movable_rocks.sort_unstable_by_key(|&(x, _)| x);

    let mut has_changed = true;
    while has_changed {
        has_changed = false;
        let mut temp_copy = movable_rocks.clone();
        for rock in &mut temp_copy {
            if rock.0 == 0
                || stable_rocks.contains(&(rock.0 - 1, rock.1))
                || movable_rocks.contains(&(rock.0 - 1, rock.1))
            {
                ()
            } else {
                has_changed = true;
                rock.0 -= 1;
            }
        }
        movable_rocks = temp_copy;
    }

    let max_x = stable_rocks.iter().max_by_key(|(x, _)| x).unwrap().0;

    movable_rocks
        .iter()
        .map(|&(x, _)| max_x - x + 1)
        .sum::<usize>()
        .to_string()
}

fn part_b(input: &str) -> String {
    String::new()
}

fn get_rocks_from_input(input: &str, input_c: char) -> Vec<Pos> {
    let mut rocks = Vec::new();
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c == input_c {
                rocks.push((i, j));
            };
        });
    });
    rocks
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day14.txt");
    const _TESTRESULT_A: &str = "136";
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
