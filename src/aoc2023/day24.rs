const INPUT: &str = include_str!("inputs/day24.txt");

use std::ops::Mul;
use std::time::Instant;

use nalgebra as na;
use nalgebra::{Matrix3, Matrix6, Vector3, Vector6};

struct Hailstone {
    p: Vector3<f64>,
    v: Vector3<f64>,
}

pub fn run() {
    let start = Instant::now();
    println!(
        "{}",
        part_a(INPUT, 200_000_000_000_000., 400_000_000_000_000.)
    );
    let elapsed1 = start.elapsed();
    println!("{}", part_b(INPUT));
    let elapsed2 = start.elapsed();

    println!("Part 1: {elapsed1:?}");
    println!("Part 2: {elapsed2:?}");
}

fn part_a(input: &str, lower_bound: f64, upper_bound: f64) -> String {
    let hailstones: Vec<Hailstone> = read_input(input);

    let mut crossings_counter = 0;

    for i in 0..hailstones.len() {
        for j in 0..i {
            let h1 = &hailstones[i];
            let h2 = &hailstones[j];

            let denom = h1.v[0].mul_add(h2.v[1], -h1.v[1] * h2.v[0]);

            let u = (h1.p[0] - h2.p[0]).mul_add(-h1.v[1], (h1.p[1] - h2.p[1]) * (h1.v[0])) / denom;
            if !(0. <= u && u.is_normal()) {
                continue;
            }

            let t = (h1.p[0] - h2.p[0]).mul_add(-h2.v[1], (h1.p[1] - h2.p[1]) * (h2.v[0])) / denom;

            if 0. <= t
                && t.is_normal()
                && check_bounds(t.mul_add(h1.v[0], h1.p[0]), lower_bound, upper_bound)
                && check_bounds(t.mul_add(h1.v[1], h1.p[1]), lower_bound, upper_bound)
            {
                crossings_counter += 1;
            }
        }
    }

    crossings_counter.to_string()
}

fn part_b(input: &str) -> String {
    let hailstones = read_input(input);

    let h1 = &hailstones[0];
    let h2 = &hailstones[1];
    let h3 = &hailstones[2];

    assert_ne!(h1.v, h2.v);
    assert_ne!(h1.v, h3.v);
    assert_ne!(h2.v, h3.v);

    let rhs: Vector6<f64> = {
        let top: Vector3<f64> = h2.p.cross(&h2.v) - h1.p.cross(&h1.v);

        let bot: Vector3<f64> = h3.p.cross(&h3.v) - h1.p.cross(&h1.v);

        top.insert_fixed_rows::<3>(3, 0.) + bot.insert_fixed_rows::<3>(0, 0.)
    };
    let lhs: Matrix6<f64> = {
        let top_left: Matrix3<f64> = (h1.v - h2.v).cross_matrix();
        let top_right: Matrix3<f64> = (h1.p - h2.p).cross_matrix();
        let top: na::Matrix3x6<f64> =
            top_left.insert_fixed_columns::<3>(3, 0.) + top_right.insert_fixed_columns::<3>(0, 0.);

        let bot_left = (h1.v - h3.v).cross_matrix();
        let bot_right = (h1.p - h3.p).cross_matrix();
        let bot =
            bot_left.insert_fixed_columns::<3>(3, 0.) + bot_right.insert_fixed_columns::<3>(0, 0.);

        top.insert_fixed_rows::<3>(3, 0.) + bot.insert_fixed_rows::<3>(0, 0.)
    };

    lhs.try_inverse()
        .unwrap()
        .mul(rhs)
        .remove_fixed_rows::<3>(3)
        .sum()
        .to_string()
}

fn read_input(input: &str) -> Vec<Hailstone> {
    input.lines().map(read_line).collect()
}

fn read_line(line: &str) -> Hailstone {
    let t: Vec<f64> = line
        .split(&[',', '@'])
        .filter_map(|s| s.trim().parse::<f64>().ok())
        .collect();
    Hailstone {
        p: Vector3::from_column_slice(&t[..3]),
        v: Vector3::from_column_slice(&t[3..]),
    }
}

fn check_bounds(x: f64, lower_bound: f64, upper_bound: f64) -> bool {
    x.is_normal() && lower_bound <= x && x <= upper_bound
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day24.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST, 7., 27.), "2");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), "46.99999999999999"); // = 47
    }
}
