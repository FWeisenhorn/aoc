use std::collections::{BinaryHeap, HashMap};

type Pos = (usize, usize);

const INPUT: &str = include_str!("inputs/day17.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}
fn part_a(input: &str) -> String {
    dijkstra(input, 1, 3)
}

fn part_b(input: &str) -> String {
    dijkstra(input, 4, 10)
}

fn dijkstra(input: &str, min_step: u8, max_step: u8) -> String {
    let heat_map: Vec<&[u8]> = input.lines().map(str::as_bytes).collect::<Vec<_>>();

    let max_x = heat_map.len();
    let max_y = heat_map[0].len();

    let mut expand_queue = BinaryHeap::<(i32, Key)>::new();
    let mut cache = HashMap::<Key, i32>::new();

    expand_queue.push((
        0,
        Key {
            pos: (0, 0),
            dir: Direction::Right,
        },
    ));
    expand_queue.push((
        0,
        Key {
            pos: (0, 0),
            dir: Direction::Down,
        },
    ));

    while let Some(cur) = expand_queue.pop() {
        if cur.1.pos == (max_x - 1, max_y - 1) {
            return (-cur.0).to_string();
        }

        expand_if_able(
            &mut expand_queue,
            &mut cache,
            &cur,
            &heat_map,
            min_step,
            max_step,
        );
    }

    unreachable!()
}

fn expand_if_able(
    to_expand: &mut BinaryHeap<(i32, Key)>,
    cache: &mut HashMap<Key, i32>,
    cur: &(i32, Key),
    heat_map: &[&[u8]],
    min_step: u8,
    max_step: u8,
) {
    if cache.get(&cur.1).is_some_and(|&c| c > cur.0) {
        return;
    }

    for new_dir in [cur.1.dir.turn_left(), cur.1.dir.turn_right()] {
        let mut new_cost = cur.0;

        for n in 1..=max_step {
            if let Some(new_pos) = new_dir.make_steps(cur.1.pos, n) {
                if new_pos.0 < heat_map.len() && new_pos.1 < heat_map[0].len() {
                    new_cost -= i32::from(heat_map[new_pos.0][new_pos.1] - b'0');

                    if n < min_step {
                        continue;
                    }

                    let new_key = Key {
                        pos: new_pos,
                        dir: new_dir,
                    };

                    if new_cost > *cache.get(&new_key).unwrap_or(&i32::MIN) {
                        cache.insert(new_key, new_cost);
                        to_expand.push((new_cost, new_key));
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum Direction {
    Up,    // -x
    Left,  // -y
    Down,  // +x
    Right, // +y
}

impl Direction {
    const fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Left => Self::Up,
            Self::Down => Self::Left,
            Self::Right => Self::Down,
        }
    }

    const fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    const fn make_steps(self, x: Pos, n: u8) -> Option<Pos> {
        let n = n as usize;
        match self {
            Self::Up if x.0 >= n => Some((x.0 - n, x.1)),
            Self::Left if x.1 >= n => Some((x.0, x.1 - n)),
            Self::Down => Some((x.0 + n, x.1)),
            Self::Right => Some((x.0, x.1 + n)),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct Key {
    pos: Pos,
    dir: Direction,
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day17.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), "102");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), "94");
    }
}
