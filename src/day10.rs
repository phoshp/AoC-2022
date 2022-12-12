const DATA: &str = include_str!("../data/day10.txt");

fn part1() -> i32 {
	let mut saved = Vec::new();
	let mut cycles = 1;
	let mut reg_x = 1;
	let mut check = 20;

	let mut check_cycle = |reg| {
		if cycles == check && check < 260 {
			saved.push(cycles * reg);
			check += 40;
		}
		cycles += 1;
	};

	for ins in DATA.lines() {
		match &ins[..4] {
			"addx" =>  {
				check_cycle(reg_x);
				check_cycle(reg_x);
				reg_x += ins[5..].parse::<i32>().unwrap();
			}
			"noop" => check_cycle(reg_x),
			_ => unreachable!()
		}
	}
	saved.iter().sum()
}

fn part2() -> String {
	let mut crt = Vec::new();
	let mut cycles = 1;
	let mut reg_x = 1;

	let mut check_cycle = |reg| {
		let d: i32 = reg - ((cycles - 1) % 40);
		if d.abs() <= 1 {
			crt.push('#');
		} else {
			crt.push(' ');
		}
		if cycles % 40 == 0 {
			crt.push('\n');
		}
		cycles += 1;
	};

	for ins in DATA.lines() {
		match &ins[..4] {
			"addx" =>  {
				check_cycle(reg_x);
				check_cycle(reg_x);
				reg_x += ins[5..].parse::<i32>().unwrap();
			}
			"noop" => check_cycle(reg_x),
			_ => unreachable!()
		}
	}
	crt.iter().collect()
}

pub fn print_result() {
	println!("Day10 Part1 Result: {}", part1());
	println!("Day10 Part2 Result:\n{}", part2());
}