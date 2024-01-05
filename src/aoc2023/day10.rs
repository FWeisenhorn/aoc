use crate::utils::{
    direction::{Direction, NEIGHBOURS},
    pos::Pos,
};
use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("inputs/day10.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    let m: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let max_x = m.len();
    let max_y = m[0].len();

    let start = {
        let x_ = m.iter().position(|v| v.contains(&b'S')).unwrap();
        let y_ = m[x_].iter().position(|x| x == &b'S').unwrap();
        Pos { x: x_, y: y_ }
    };

    let mut i: usize = 0;

    let mut walker = start;
    let mut walker_dir = find_initial_direction(&m, &start, max_x, max_y);

    // go through the loop until we are back at the start
    loop {
        i += 1;

        if let Some(t) = walker.steps_checked(walker_dir, 1, max_x, max_y) {
            walker = t;
            match (walker_dir, m[t.x][t.y]) {
                (_, b'S') => {
                    break;
                }
                (Direction::Up | Direction::Down, b'|')
                | (Direction::Left | Direction::Right, b'-') => (),

                (Direction::Up, b'7')
                | (Direction::Down, b'L')
                | (Direction::Left, b'F')
                | (Direction::Right, b'J') => {
                    walker_dir.turn_assign_left();
                }
                (Direction::Up, b'F')
                | (Direction::Down, b'J')
                | (Direction::Left, b'L')
                | (Direction::Right, b'7') => {
                    walker_dir.turn_assign_right();
                }
                _ => unreachable!(),
            }
        }
    }
    (i / 2).to_string()
}

fn part_b(input: &str) -> String {
    let m: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let max_x = m.len();
    let max_y = m[0].len();

    let start = {
        let x_ = m.iter().position(|v| v.contains(&b'S')).unwrap();
        let y_ = m[x_].iter().position(|x| x == &b'S').unwrap();
        Pos { x: x_, y: y_ }
    };

    let mut walker = start;
    let mut walker_dir = find_initial_direction(&m, &start, max_x, max_y);

    let mut visited: HashSet<Pos> = HashSet::new();
    let mut passed_rhs: HashSet<Pos> = HashSet::new();
    let mut passed_lhs: HashSet<Pos> = HashSet::new();

    let mut turn_counter: i32 = 0; // will be > 0 if the route was walked in a right turn

    while let Some(t) = walker.steps_checked(walker_dir, 1, max_x, max_y) {
        visited.insert(t);
        walker = t;

        match (walker_dir, m[t.x][t.y]) {
            (_, b'S') => {
                break;
            }
            //
            (Direction::Up | Direction::Down, b'|')
            | (Direction::Left | Direction::Right, b'-') => {
                add_neighbour(&walker, walker_dir.turn_left(), &mut passed_lhs);
                add_neighbour(&walker, walker_dir.turn_right(), &mut passed_rhs);
            }

            (Direction::Up, b'7')
            | (Direction::Down, b'L')
            | (Direction::Left, b'F')
            | (Direction::Right, b'J') => {
                add_neighbour(&walker, walker_dir, &mut passed_rhs);
                add_neighbour(&walker, walker_dir.turn_right(), &mut passed_rhs);
                walker_dir.turn_assign_left();
                turn_counter -= 1;
            }
            (Direction::Up, b'F')
            | (Direction::Down, b'J')
            | (Direction::Left, b'L')
            | (Direction::Right, b'7') => {
                add_neighbour(&walker, walker_dir, &mut passed_lhs);
                add_neighbour(&walker, walker_dir.turn_left(), &mut passed_lhs);
                walker_dir.turn_assign_right();
                turn_counter += 1;
            }

            _ => unreachable!(),
        }
    }

    expand_queue(
        (if turn_counter > 0 {
            passed_rhs
        } else {
            passed_lhs
        })
        .difference(&visited)
        .copied()
        .collect(),
        &visited,
        max_x,
        max_y,
    )
    .len()
    .to_string()
}

fn find_initial_direction(m: &[&[u8]], start: &Pos, max_x: usize, max_y: usize) -> Direction {
    if let Some(t) = start.steps_checked(Direction::Down, 1, max_x, max_y) {
        if matches!(m[t.x][t.y], b'|' | b'J' | b'L') {
            return Direction::Down;
        }
    }

    if let Some(t) = start.steps_checked(Direction::Up, 1, max_x, max_y) {
        if matches!(m[t.x][t.y], b'|' | b'F' | b'7') {
            return Direction::Up;
        }
    }

    if let Some(t) = start.steps_checked(Direction::Right, 1, max_x, max_y) {
        if matches!(m[t.x][t.y], b'-' | b'J' | b'7') {
            return Direction::Right;
        }
    }
    unreachable!()
}

#[inline]
fn add_neighbour(cur_pos: &Pos, d: Direction, s: &mut HashSet<Pos>) {
    if let Some(neigh) = cur_pos.steps(d, 1) {
        s.insert(neigh);
    }
}

fn expand_queue(
    mut to_expand: VecDeque<Pos>,
    visited: &HashSet<Pos>,
    max_x: usize,
    max_y: usize,
) -> HashSet<Pos> {
    let mut expanded = HashSet::<Pos>::new();

    while let Some(cur) = to_expand.pop_front() {
        NEIGHBOURS
            .iter()
            .filter_map(|&d| cur.steps_checked(d, 1, max_x, max_y))
            .for_each(|p| {
                if !(expanded.contains(&p) || to_expand.contains(&p) || visited.contains(&p)) {
                    to_expand.push_back(p);
                }
            });

        expanded.insert(cur);
    }

    expanded
}

#[cfg(test)]
mod tests {
    const _TEST_A1: &str = include_str!("tests/day10-a1.txt");
    const _TEST_A2: &str = include_str!("tests/day10-a2.txt");

    const _TEST_B1: &str = include_str!("tests/day10-b1.txt");
    const _TEST_B2: &str = include_str!("tests/day10-b2.txt");
    const _TEST_B3: &str = include_str!("tests/day10-b3.txt");

    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST_A1), "4");
        assert_eq!(part_a(_TEST_A2), "8");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST_B1), "4");
        assert_eq!(part_b(_TEST_B2), "8");
        assert_eq!(part_b(_TEST_B3), "10");
    }
}
