use std::collections::BinaryHeap;

use itertools::Itertools;

type Pos = (i32, i32);

const DIAGONALS: [Pos; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Default, Clone, Debug)]
struct Node {
    x: i32,
    y: i32,
    h: u8,
}

fn neighbours(coord: Pos, grid: &[Vec<Node>], cond: impl Fn(u8, u8) -> bool) -> Vec<Pos> {
	let height = grid[coord.1 as usize][coord.0 as usize].h;
	DIAGONALS
		.iter()
		.filter_map(|(dy, dx)| {
			let a = (coord.0 + dx) as usize;
			let b = (coord.1 + dy) as usize;
			let opt = grid.get(b).and_then(|o| o.get(a));

			if let Some(other) = opt {
				if cond(height, other.h) {
					return Some((other.x, other.y));
				}
			}
			None
		})
		.collect_vec()
}

fn find_path(grid: &[Vec<Node>], start: Pos, goal: impl Fn(Pos) -> bool, cond: impl Fn(u8, u8) -> bool) -> i32 {
	let mut closed = Vec::new();
	let mut open = BinaryHeap::new();
	open.push((0, start.0, start.1));

	while let Some((step, x, y)) = open.pop() {
		if closed.contains(&(x, y)) {
			continue;
		}
		closed.push((x, y));

		if goal((x, y)) {
			return -step; // using max-heap queue as a min-heap
		}
		for neighbor in neighbours((x, y), grid, &cond) {
			open.push((step - 1, neighbor.0, neighbor.1));
		}
	}
	0
}

fn create_grid() -> (Pos, Pos, Vec<Vec<Node>>) {
	let mut start = (0, 0);
	let mut end = (0, 0);
	let grid = include_str!("../data/day12.txt")
        .lines()
        .map(|x| x.as_bytes())
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, e)| {
                    if *e == b'S' {
                        start = (x as i32, y as i32);
                        b'a'
                    } else if *e == b'E' {
                        end = (x as i32, y as i32);
                        b'z'
                    } else {
                        *e
                    }
                })
				.enumerate()
				.map(|(x, h)| Node { x: x as i32, y: y as i32, h })
                .collect_vec()
        })
        .collect_vec();
	(start, end, grid)
}

fn part1() -> i32 {
    let (start, end, grid) = create_grid();
	find_path(&grid, start, |pos| pos == end, |ch, oh| oh <= ch + 1)
}

fn part2() -> i32 {
    let (_, end, grid) = create_grid();
	find_path(&grid, end, |(x, y)| grid[y as usize][x as usize].h == b'a', |ch, oh| oh >= ch - 1)
}

pub fn print_result() {
    println!("Day12 Part1 Result: {}", part1());
    println!("Day12 Part2 Result: {}", part2());
}
