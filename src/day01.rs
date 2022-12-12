use itertools::Itertools;

const DATA: &str = include_str!("../data/day1.txt");

fn part1() -> u32 {
	DATA.replace('\r', "")
		.split("\n\n")
		.map(|x| x.lines().map(|y| y.parse::<u32>().unwrap()).sum())
		.max().unwrap()
}

fn part2() -> u32 {
	DATA.replace('\r', "")
		.split("\n\n")
		.map(|x| x.lines().map(|y| y.parse::<u32>().unwrap()).sum::<u32>())
		.sorted().rev().take(3).sum()
}

pub fn print_result() {
	println!("Day1 Part1 Result: {}", part1());
	println!("Day1 Part2 Result: {}", part2());
}