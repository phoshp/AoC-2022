use itertools::Itertools;

const DATA: &str = include_str!("../data/day2.txt");

fn rps_score(x: &str) -> u32 {
	match x {
		"X" => 1,
		"Y" => 2,
		"Z" => 3,
		_ => unreachable!()
	}
}

fn part1() -> u32 {
	DATA.lines()
		.map(|x| {
			let t = x.split_whitespace()
				.collect_tuple::<(&str, &str)>().unwrap();
			let y = match t.0 {
				"A" => "X",
				"B" => "Y",
				"C" => "Z",
				_ => unreachable!()
			};
			rps_score(t.1) + match (y, t.1) {
				("X", "X") | ("Y", "Y") | ("Z", "Z") => 3,
				("X", "Y") | ("Y", "Z") | ("Z", "X") => 6,
				("X", "Z") | ("Y", "X") | ("Z", "Y") => 0,
				_ => unreachable!()
			}
		}).sum::<u32>()	
}

fn part2() -> u32 {
	DATA.lines()
		.map(|x| {
			let t = x.split_whitespace()
				.collect_tuple::<(&str, &str)>().unwrap();
			let y = match t.0 {
				"A" => "X",
				"B" => "Y",
				"C" => "Z",
				_ => unreachable!()
			};
			let x = match (t.1, y) {
				("X", "X") => "Z",
				("X", "Y") => "X",
				("X", "Z") => "Y",
				("Y", "X") => "X",
				("Y", "Y") => "Y",
				("Y", "Z") => "Z",
				("Z", "X") => "Y",
				("Z", "Y") => "Z",
				("Z", "Z") => "X",
				_ => unreachable!()
			};
			rps_score(x) + match (y, x) {
				("X", "X") | ("Y", "Y") | ("Z", "Z") => 3,
				("X", "Y") | ("Y", "Z") | ("Z", "X") => 6,
				("X", "Z") | ("Y", "X") | ("Z", "Y") => 0,
				_ => unreachable!()
			}
		}).sum::<u32>()	
}

pub fn print_result() {
	println!("Day2 Part1 Result: {}", part1());
	println!("Day2 Part2 Result: {}", part2());
}