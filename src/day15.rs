use std::{collections::HashMap, ops::RangeInclusive};

const DATA: &str = include_str!("../data/day15.txt");

type Pos = (i64, i64);

fn manhattan_dist(src: Pos, dst: Pos) -> i64 {
    (dst.0 - src.0).abs() + (dst.1 - src.1).abs()
}

fn blocked_ranges(y: i64, sensor: Pos, beacon: Pos) -> Option<RangeInclusive<i64>> {
    let d = manhattan_dist(sensor, beacon);
    let d2 = d - (sensor.1 - y).abs();
    if d2 <= 0 {
        return None;
    }
    Some((sensor.0 - d2)..=(sensor.0 + d2))

}

fn create_grid() -> HashMap<Pos, Pos> {
    DATA.lines()
        .map(|line| {
            let (s, b) = line.split_once(": ").unwrap();
            let (scoord, bcoord) = (
                s.split_once("at ").unwrap().1.split_once(", ").unwrap(),
                b.split_once("at ").unwrap().1.split_once(", ").unwrap(),
            );
            let sensor_xy = (
                scoord.0[2..].parse::<i64>().unwrap(),
                scoord.1[2..].parse::<i64>().unwrap(),
            );
            let beacon_xy = (
                bcoord.0[2..].parse::<i64>().unwrap(),
                bcoord.1[2..].parse::<i64>().unwrap(),
            );
            (sensor_xy, beacon_xy)
        })
        .collect()
}

fn part1() -> usize {
    let grid = create_grid();
    let mut max_range: Option<RangeInclusive<i64>> = None;
    for (&sensor, &beacon) in grid.iter() {
        if let Some(r) = blocked_ranges(2000000, sensor, beacon) {
            if let Some(c) = max_range {
                max_range = Some((*r.start().min(c.start()))..=(*r.end().max(c.end())));
            } else {
                max_range = Some(r);
            }
        }
    }
    let mut blocked = 0;
    for i in max_range.unwrap() {
        let pos = (i, 2000000);
        for (&s, &b) in grid.iter() {
            let d = manhattan_dist(s, b);
            let d2 = manhattan_dist(s, pos);
            if d2 <= d && pos != b && pos != s {
                blocked += 1;
                break;
            }
        }
    }
    blocked
}

fn is_valid(p: Pos) -> bool {
    p.0 >= 0 && p.0 <= 4_000_000 && p.1 >= 0 && p.1 <= 4_000_000
}

fn is_free(grid: &HashMap<Pos, Pos>, target: Pos) -> bool {
    grid.iter()
        .filter(|(&s2, &b2)| {
            let def = manhattan_dist(s2, b2);
            manhattan_dist(target, s2) > def
        })
        .count()
        == grid.len()
}

fn part2() -> i64 {
	let grid = create_grid();
    let mut gap = None;
    'main: for (&s, &b) in grid.iter() {
        let d = manhattan_dist(s, b) + 1;
        let mut dx = 1;
        let mut dy = 1;
        let mut curr = (s.0 + 1, s.1 - d + 1);
        loop {
            if is_valid(curr) && is_free(&grid, curr) {
                gap = Some(curr);
                break 'main;
            }

            if curr.1 == s.1 + d {
                dy = -1;
            } else if curr.1 == s.1 - d {
                break;
            }
            if curr.0 == s.0 + d {
                dx = -1;
            } else if curr.0 == s.0 - d {
                dx = 1;
            }
            curr.0 += dx;
            curr.1 += dy;
        }
    }
    let result = gap.unwrap();
    result.0 * 4_000_000 + result.1
}

pub fn print_result() {
    println!("Day15 Part1 Result: {}", part1());
    println!("Day15 Part2 Result: {}", part2());
}
