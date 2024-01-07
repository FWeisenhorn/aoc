use crate::utils::{
    direction::{Direction, NEIGHBOURS},
    pos::Pos,
};
use std::{collections::HashMap, time::Instant};

const INPUT: &str = include_str!("inputs/day23.txt");

pub fn run() {
    // "Adapted" from https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/23.rs
    let start = Instant::now();
    println!("{}", part_a(INPUT));
    let elapsed1 = start.elapsed();
    println!("{}", part_b(INPUT));
    let elapsed2 = start.elapsed();

    println!("Part 1 : {elapsed1:?}");
    println!("Part 2 : {elapsed2:?}");
}

fn part_a(input: &str) -> String {
    let area: Vec<_> = input.lines().map(str::as_bytes).collect();

    solve(&area, false).to_string()
}

fn part_b(input: &str) -> String {
    let area: Vec<_> = input.lines().map(str::as_bytes).collect();

    solve(&area, true).to_string()
}

fn solve(area: &[&[u8]], part2: bool) -> usize {
    let max_x = area.len();
    let max_y = area[0].len();

    // let mut paths = vec![];

    let mut graph = HashMap::<Pos, Vec<(Pos, usize)>>::new();

    for x_ in 0..max_x {
        for y_ in 0..max_y {
            let pos = Pos { x: x_, y: y_ };

            if area[pos.x][pos.y] == b'#' {
                continue;
            }

            let val = graph.entry(pos).or_default();

            if matches!(area[pos.x][pos.y], b'^' | b'v' | b'<' | b'>') && !part2 {
                let d = match area[pos.x][pos.y] {
                    b'^' => Direction::Up,
                    b'v' => Direction::Down,
                    b'<' => Direction::Left,
                    b'>' => Direction::Right,
                    _ => unreachable!(),
                };

                pos.steps_checked(d, 1, max_x, max_y)
                    .filter(|new_p| area[new_p.x][new_p.y] != b'#')
                    .and_then(|new_p| graph.insert(pos, vec![(new_p, 1)]));
            } else {
                NEIGHBOURS
                    .iter()
                    .filter_map(|&d| pos.steps_checked(d, 1, max_x, max_y))
                    .filter(|new_p| area[new_p.x][new_p.y] != b'#')
                    .for_each(|new_p| val.push((new_p, 1)));
            }
        }
    }

    let corridors: Vec<Pos> = graph
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(&node, _)| node)
        .collect();

    for pos in corridors {
        let neighbors = graph.remove(&pos).unwrap();
        let (p1, d1) = neighbors[0];
        let (p2, d2) = neighbors[1];

        let n1 = graph.get_mut(&p1).unwrap();
        if let Some(i) = n1.iter().position(|&(p_, _)| p_ == pos) {
            n1[i] = (p2, d1 + d2);
        }

        let n2 = graph.get_mut(&p2).unwrap();
        if let Some(i) = n2.iter().position(|&(p_, _)| p_ == pos) {
            n2[i] = (p1, d1 + d2);
        }
    }

    // convert: hashmap<pos, vec[pos]> -> index hashmap <pos, i> + vec <j>
    let indexes: HashMap<&Pos, usize> = graph.keys().enumerate().map(|(i, pos)| (pos, i)).collect();

    let mut idx_graph = vec![vec![]; graph.len()];

    for (pos, neighbors) in &graph {
        idx_graph[indexes[pos]] = neighbors.iter().map(|&(p, d)| (indexes[&p], d)).collect();
    }

    let finish = indexes[&Pos {
        x: area.len() - 1,
        y: area[0].len() - 2,
    }];
    dfs(
        &idx_graph,
        &mut vec![false; idx_graph.len()],
        finish,
        indexes[&Pos { x: 0, y: 1 }],
    )
    .unwrap()
}

fn dfs(
    idx_graph: &Vec<Vec<(usize, usize)>>,
    visited: &mut [bool],
    finish: usize,
    curr: usize,
) -> Option<usize> {
    if curr == finish {
        return Some(0);
    }

    let mut max_d = None;
    for &next in &idx_graph[curr] {
        if visited[next.0] {
            continue;
        }
        visited[next.0] = true;
        if let Some(new_d) = dfs(idx_graph, visited, finish, next.0) {
            max_d = max_d.max(Some(next.1 + new_d));
        }
        visited[next.0] = false;
    }
    max_d
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day23.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), "94");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), "154");
    }
}
