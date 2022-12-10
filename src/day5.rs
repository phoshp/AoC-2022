const DATA: &str = include_str!("../data/day5.txt");

fn part1() -> String {
	let data = DATA.replace('\r', "");
	let (img, cmds) = data.split_once("\n\n").unwrap();
	let mut columns: Vec<Vec<char>> = Vec::new();

	img.lines().rev()
		.for_each(|line| {
			let mut prev = None;
			line.chars().enumerate().for_each(|(i, set)| {
				if prev == Some('[') {
					let index = ((i - 1) / 4) as usize;
					if let Some(v) = columns.get_mut(index) {
						v.push(set);
					} else {
						columns.insert(index, vec![set]);
					}
				}
				prev = Some(set);
			});
	});
	cmds.lines()
		.for_each(|line| {
			let mut parts = line.split_whitespace();
			let count = parts.nth(1).unwrap().parse::<u32>().unwrap();
			let src_col = parts.nth(1).unwrap().parse::<usize>().unwrap() - 1;
			let dst_col = parts.nth(1).unwrap().parse::<usize>().unwrap() - 1;

			for _ in 0..count {
				let c = columns[src_col].pop().unwrap();
				columns[dst_col].push(c);
			}
		});
	columns.iter()
		.map(|v| v.last().unwrap())
		.collect()
}

fn part2() -> String {
	let data = DATA.replace('\r', "");
	let (img, cmds) = data.split_once("\n\n").unwrap();
	let mut columns: Vec<Vec<char>> = Vec::new();

	img.lines().rev()
		.for_each(|line| {
			let mut prev = None;
			line.chars().enumerate().for_each(|(i, set)| {
				if prev == Some('[') {
					let index = ((i - 1) / 4) as usize;
					if let Some(v) = columns.get_mut(index) {
						v.push(set);
					} else {
						columns.insert(index, vec![set]);
					}
				}
				prev = Some(set);
			});
	});
	cmds.lines()
		.for_each(|line| {
			let mut parts = line.split_whitespace();
			let count = parts.nth(1).unwrap().parse::<u32>().unwrap();
			let src_col = parts.nth(1).unwrap().parse::<usize>().unwrap() - 1;
			let dst_col = parts.nth(1).unwrap().parse::<usize>().unwrap() - 1;

			let mut crates = Vec::new();
			for _ in 0..count {
				crates.push(columns[src_col].pop().unwrap());
				
			}
			crates.reverse();
			for cratee in crates {
				columns[dst_col].push(cratee);
			}
		});
	columns.iter()
		.map(|v| v.last().unwrap())
		.collect()
}

pub fn print_result() {
	println!("Day5 Part1 Result: {}", part1());
	println!("Day5 Part2 Result: {}", part2());
}