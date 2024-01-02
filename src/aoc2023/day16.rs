use crate::utils::{direction::Direction, pos::Pos};
use std::collections::HashMap;

const INPUT: &str = include_str!("inputs/day16.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    let grid = input.lines().map(str::as_bytes).collect::<Vec<_>>();

    light_up_grid(&grid, Pos { x: 0, y: 0 }, Direction::Right).to_string()
}

fn part_b(input: &str) -> String {
    let grid = input.lines().map(str::as_bytes).collect::<Vec<_>>();

    let a = (0..grid[0].len())
        .map(|y_| light_up_grid(&grid, Pos { x: 0, y: y_ }, Direction::Down))
        .max()
        .unwrap();

    let b = (0..grid[0].len())
        .map(|y_| {
            light_up_grid(
                &grid,
                Pos {
                    x: grid.len() - 1,
                    y: y_,
                },
                Direction::Down,
            )
        })
        .max()
        .unwrap();

    let c = (0..grid.len())
        .map(|x_| light_up_grid(&grid, Pos { x: x_, y: 0 }, Direction::Right))
        .max()
        .unwrap();

    let d = (0..grid.len())
        .map(|x_| {
            light_up_grid(
                &grid,
                Pos {
                    x: x_,
                    y: grid[0].len() - 1,
                },
                Direction::Left,
            )
        })
        .max()
        .unwrap();

    [a, b, c, d].into_iter().max().unwrap().to_string()
}

fn light_up_grid(grid: &[&[u8]], starting_pos: Pos, starting_dir: Direction) -> usize {
    let mut energized: HashMap<Pos, Vec<Direction>> = HashMap::new();
    let mut beam_ends: Vec<(Pos, Direction)> = vec![(starting_pos, starting_dir)];

    while let Some((cur_pos, cur_dir)) = beam_ends.pop() {
        if cur_pos.x >= grid.len() || cur_pos.y >= grid[0].len() {
            continue;
        }
        if energized
            .get(&cur_pos)
            .is_some_and(|v| v.contains(&cur_dir))
        {
            continue;
        }

        energized.entry(cur_pos).or_default().push(cur_dir);

        match grid[cur_pos.x][cur_pos.y] {
            b'/' => {
                push_pos_and_dir(&mut beam_ends, cur_pos, cur_dir.mirror_slash());
            }
            b'\\' => {
                push_pos_and_dir(&mut beam_ends, cur_pos, cur_dir.mirror_backslash());
            }
            b'-' if cur_dir.is_vertical() => {
                push_pos_and_dir(&mut beam_ends, cur_pos, Direction::Left);
                push_pos_and_dir(&mut beam_ends, cur_pos, Direction::Right);
            }
            b'|' if cur_dir.is_horizontal() => {
                push_pos_and_dir(&mut beam_ends, cur_pos, Direction::Up);
                push_pos_and_dir(&mut beam_ends, cur_pos, Direction::Down);
            }
            _ => {
                push_pos_and_dir(&mut beam_ends, cur_pos, cur_dir);
            }
        }
    }

    energized.len()
}

fn push_pos_and_dir(queue: &mut Vec<(Pos, Direction)>, pos: Pos, dir: Direction) {
    if let Some(new_pos) = pos.steps(dir, 1) {
        queue.push((new_pos, dir));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day16.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), "46");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), "51");
    }
}
