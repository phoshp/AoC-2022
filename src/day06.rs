const DATA: &str = include_str!("../data/day06.txt");

fn part_common<const T: usize>() -> usize {
	let mut marker = "".to_string();
	let chars = DATA.as_bytes();

	let mut i = 0;
	let mut skip = 0;
	loop {
		if i >= chars.len() || marker.len() == T {
			break;
		}
		let next = chars[i] as char;

		if let Some(v) = marker.find(next) {
			i = skip + v + 1;
			skip += v + 1;
			marker.clear();
		} else {
			marker.push(next);
			i += 1;
		}
	}
	i
}

pub fn print_result() {
	println!("Day6 Part1 Result: {}", part_common::<4>());
	println!("Day6 Part2 Result: {}", part_common::<14>());
}