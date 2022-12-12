use itertools::Itertools;

const DATA: &str = include_str!("../data/day03.txt");

fn part1() -> i16 {
	DATA.lines()
		.map(|x| x.as_bytes().split_at(x.len() / 2))
		.map(|(f, s)| f
			.iter()
			.filter(|x| s.contains(x))
			.map(|x| if *x >= b'a' {
				(*x - b'a') as i16 + 1
			} else {
				(*x - b'A') as i16 + 27
			})
			.next().unwrap()
		).sum()
}

fn part2() -> i16 {
	DATA.lines()
		.collect_vec()
		.chunks(3)
		.map(|chunk| chunk[0].chars()
			.find(|c| chunk[1].contains(*c) && chunk[2].contains(*c)).unwrap()
		)
		.map(|x| x as u8)
		.map(|x| if x >= b'a' {
			(x - b'a') as i16 + 1
		} else {
			(x - b'A') as i16 + 27
		}).sum::<i16>()
}

pub fn print_result() {
	println!("Day3 Part1 Result: {}", part1());
	println!("Day3 Part2 Result: {}", part2());
}