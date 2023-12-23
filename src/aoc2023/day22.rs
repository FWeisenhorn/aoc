use std::cmp::{max, min};
use std::collections::HashSet;

type Pos = [u32; 3];
type Brick = Vec<Pos>;

const INPUT: &str = include_str!("inputs/day22.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    let mut bricks: Vec<Brick> = input.lines().map(create_brick_from_string).collect();
    bricks.sort_unstable_by_key(|v| v[0][2]);
    move_bricks_to_floor(&mut bricks);

    let neighbours = count_neighbours(&bricks);

    let mut safe_to_remove = 0;

    for i in 0..bricks.len() {
        if neighbours[i].1.iter().all(|&j| neighbours[j].0.len() > 1) {
            safe_to_remove += 1;
        }
    }

    safe_to_remove.to_string()
}

fn part_b(input: &str) -> String {
    let mut bricks: Vec<Brick> = input.lines().map(create_brick_from_string).collect();
    bricks.sort_unstable_by_key(|v| v[0][2]);
    move_bricks_to_floor(&mut bricks);
    bricks.sort_unstable_by_key(|v| v[0][2]);

    let neighbours = count_neighbours(&bricks);

    let mut out = 0;

    for i in 0..bricks.len() {
        let mut falling = HashSet::new();
        falling.insert(i);

        for (j, brick) in neighbours.iter().enumerate().skip(i + 1) {
            if !brick.0.is_empty() && brick.0.is_subset(&falling) {
                falling.insert(j);
            }
        }

        out += falling.len() - 1;
    }

    out.to_string()
}

fn create_brick_from_string(input: &str) -> Brick {
    let t = input.split_once('~').unwrap();

    let ([x1, y1, z1], [x2, y2, z2]) = (create_pos_from_string(t.0), create_pos_from_string(t.1));

    (min(x1, x2)..=max(x1, x2))
        .flat_map(|x| {
            (min(y1, y2)..=max(y1, y2))
                .flat_map(move |y| (min(z1, z2)..=max(z1, z2)).map(move |z| [x, y, z]))
        })
        .collect()
}

fn create_pos_from_string(input: &str) -> Pos {
    let t: Vec<_> = input.split(',').map(|s| s.parse().unwrap()).collect();
    [t[0], t[1], t[2]]
}

fn move_bricks_to_floor(bricks: &mut Vec<Brick>) {
    for i in 0..bricks.len() {
        let bricks_pos = bricks.clone().drain(0..i).flatten().collect::<HashSet<_>>();

        let brick_falling = bricks.get_mut(i).unwrap();

        loop {
            if brick_falling.iter().any(|&[_, _, z]| z <= 1) {
                break;
            }

            if brick_falling
                .iter()
                .any(|&[x, y, z]| bricks_pos.contains(&[x, y, z - 1]))
            {
                break;
            }

            *brick_falling = brick_falling
                .iter()
                .map(|&[x, y, z]| [x, y, z - 1])
                .collect();
        }
    }
}

fn count_neighbours(bricks: &[Brick]) -> Vec<(HashSet<usize>, HashSet<usize>)> {
    let mut neighbours: Vec<(HashSet<usize>, HashSet<usize>)> =
        vec![(HashSet::new(), HashSet::new()); bricks.len()];

    for i in 0..bricks.len() {
        for j in 0..i {
            if bricks[i]
                .iter()
                .any(|&[x, y, z]| bricks[j].contains(&[x, y, z - 1]))
            {
                neighbours[j].1.insert(i);
                neighbours[i].0.insert(j);
            }
        }
    }

    neighbours
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day22.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), "5");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), "7");
    }
}
