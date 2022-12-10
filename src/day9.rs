use std::{ops::{Add, AddAssign}, collections::HashSet};

#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
struct Point(f32, f32);

impl Add for Point {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self(self.0 + rhs.0, self.1 + rhs.1)
	}
}

impl AddAssign for Point {
	fn add_assign(&mut self, rhs: Self) {
		self.0 += rhs.0;
		self.1 += rhs.1;
	}
}

impl Point {
	fn dist(&self, other: Point) -> f32 {
		let x = other.0 - self.0;
		let y = other.1 - self.1;
		x * x + y * y
	}

	fn hash(&self) -> String {
		[self.0, self.1].map(|x| x.to_string()).join("|")
	}
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Knot {
	pos: Point
}

const DIAGONALS: [Point; 8] = [
	Point(1.0, 0.0),
	Point(0.0, 1.0),
	Point(1.0, 1.0),
	Point(-1.0, -1.0),
	Point(-1.0, 0.0),
	Point(-1.0, 1.0),
	Point(1.0, -1.0),
	Point(0.0, -1.0)
];

impl Knot {
	fn force(&mut self, mot: Point) {
		self.pos += mot;
	}

	fn update(&mut self, n: Option<&Knot>) -> bool {
		if let Some (next) = n {
			let d = self.pos.dist(next.pos);
			if d > 2.0 {
				let nearest = DIAGONALS.iter().min_by_key(|x| (self.pos + **x).dist(next.pos) as u32).unwrap();
				self.pos += *nearest;
				return true;
			}
		}
		false
	}
}

const DATA: &str = include_str!("../data/day9.txt");

fn part1() -> u32 {
	let rope = vec![Knot { pos: Point(0.5, 0.5) }; 2];
	common(rope)
}

fn part2() -> u32 {
	let mut rope = Vec::new();
	for _ in 0..10 {
		rope.push(Knot { pos: Point(0.5, 0.5) });
	}
	common(rope)
}

fn common(mut rope: Vec<Knot>) -> u32 {
	let mut tail_map: HashSet<String> = HashSet::new(); 
	tail_map.insert("0.5|0.5".to_string()); // starting point

	for line in DATA.lines() {
		let mut args = line.split_whitespace();
		let key = args.next().unwrap();
		let i = args.next().unwrap().parse::<u32>().unwrap();
		let motion = match key {
			"U" => Point(0.0, 1.0),
			"D" => Point(0.0, -1.0),
			"L" => Point(1.0, 0.0),
			"R" => Point(-1.0, 0.0),
			_ => unreachable!()
		};

		for _ in 0..i {
			rope.first_mut().unwrap().force(motion);
			let mut i = 0;
			let mut prev = None;
			for knot in rope.iter_mut() {
				if !knot.update(prev) && i > 0 {
					break;
				}
				prev = Some(knot);
				i += 1;
			}
			if i == rope.len() { // updated tail
				tail_map.insert(rope.last().unwrap().pos.hash());
			}
		}
	}
	tail_map.len() as u32
}

pub fn print_result() {
	println!("Day9 Part1 Result: {}", part1());
	println!("Day9 Part2 Result: {}", part2());
}