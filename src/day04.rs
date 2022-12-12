const DATA: &str = include_str!("../data/day4.txt");

fn part1() -> usize {
	DATA.lines()
		.map(|x| {
			let (f, s) = x.split_once(',').unwrap();
			let (fmin, fmax) = f.split_once('-').unwrap();
			let (smin, smax) = s.split_once('-').unwrap();
			(
				fmin.parse::<u32>().unwrap(),
				fmax.parse::<u32>().unwrap(),
				smin.parse::<u32>().unwrap(),
				smax.parse::<u32>().unwrap()
			)
		})
		.filter(|(fmin, fmax, smin, smax)| (fmin >= smin && fmax <= smax) || (smin >= fmin && smax <= fmax))
		.count()
}

fn part2() -> usize {
	DATA.lines()
		.map(|x| {
			let (f, s) = x.split_once(',').unwrap();
			let (fmin, fmax) = f.split_once('-').unwrap();
			let (smin, smax) = s.split_once('-').unwrap();
			(
				fmin.parse::<u32>().unwrap(),
				fmax.parse::<u32>().unwrap(),
				smin.parse::<u32>().unwrap(),
				smax.parse::<u32>().unwrap()
			)
		})
		.filter(|(fmin, fmax, smin, smax)| fmin <= smax && smin <= fmax)
		.count()
}

pub fn print_result() {
	println!("Day4 Part1 Result: {}", part1());
	println!("Day4 Part2 Result: {}", part2());
}