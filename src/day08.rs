use itertools::Itertools;

enum Direction {
	Up,
	Down,
	Left,
	Right
}

const DATA: &str = include_str!("../data/day08.txt");

fn part1() -> u32 {
	let grid = DATA
		.lines()
		.map(|x| {
			x.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()
		}).collect_vec();
	let mut visible = 0;
	for (y, row) in grid.iter().enumerate() {
		for (x, _) in row.iter().enumerate() {
			if can_see_tree(&grid, (x, y)) {
				visible += 1;
			}
		}
	}
	visible
}

fn part2() -> u32 {
	let grid = DATA
		.lines()
		.map(|x| {
			x.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()
		}).collect_vec();
	grid.iter().enumerate()
		.map(|(y, row)| row.iter().enumerate()
			.map(|(x, _)| scenic_score(&grid, (x, y)))
			.max().unwrap()
		).max().unwrap()
}

fn scenic_score(grid: &[Vec<u32>], pos: (usize, usize)) -> u32 {
	tree_by_direction(grid, pos, Direction::Up).0 *
	tree_by_direction(grid, pos, Direction::Down).0 *
	tree_by_direction(grid, pos, Direction::Left).0 *
	tree_by_direction(grid, pos, Direction::Right).0
}

fn can_see_tree(grid: &[Vec<u32>], pos: (usize, usize)) -> bool {
	pos.0 == 0 || pos.0 == (grid[0].len() - 1) ||
	pos.1 == 0 || pos.1 == (grid.len() - 1) ||
	tree_by_direction(grid, pos, Direction::Up).1 ||
	tree_by_direction(grid, pos, Direction::Down).1 ||
	tree_by_direction(grid, pos, Direction::Left).1 ||
	tree_by_direction(grid, pos, Direction::Right).1
}

fn tree_by_direction(grid: &[Vec<u32>], pos: (usize, usize), direction: Direction) -> (u32, bool) {
	let h = grid[pos.1][pos.0];
	let row = grid[pos.1].iter();
	let mut visible = 0;
	match direction {
		Direction::Left => {
			for i in (0..pos.0).rev() {
				visible += 1;
				if grid[pos.1][i] >= h {
					return (visible, false);
				}
			}
			(visible, true)
		}
		Direction::Right => {
			for i in (pos.0 + 1)..row.len() {
				visible += 1;
				if grid[pos.1][i] >= h {
					return (visible, false);
				}
			}
			(visible, true)
		}
		Direction::Up => {
			for i in (0..pos.1).rev()  {
				visible += 1;
				if grid[i][pos.0] >= h {
					return (visible, false);
				}
			}
			(visible, true)
		}
		Direction::Down => {
			for c in grid.iter().skip(pos.1 + 1) {
				visible += 1;
				if c[pos.0] >= h {
					return (visible, false);
				}
			}
			(visible, true)
		}
	}
}

pub fn print_result() {
	println!("Day8 Part1 Result: {}", part1());
	println!("Day8 Part2 Result: {}", part2());
}