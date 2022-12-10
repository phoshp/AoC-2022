const DATA: &str = include_str!("../data/day10.txt");

fn part1() -> i32 {
	let mut saved = Vec::new();
	let mut cycles = 1;
	let mut reg_x = 1;
	let mut check = 20;

	let mut check_cycle = |x, reg| {
		if x == check && check < 260 {
			saved.push(x * reg);
			check += 40;
		}
	};

	for ins in DATA.lines() {
		match &ins[..4] {
			"addx" =>  {
				check_cycle(cycles, reg_x);
				cycles += 1;
				check_cycle(cycles, reg_x);
				cycles += 1;
				reg_x += ins[5..].parse::<i32>().unwrap();

			}
			"noop" => {
				check_cycle(cycles, reg_x);
				cycles += 1;
			}
			_ => unreachable!()
		}
	}
	saved.iter().sum()
}

fn part2() -> String {
	let mut crt = Vec::new();
	let mut cycles = 1;
	let mut reg_x = 1;

	let mut check_cycle = |cycle, reg| {
		let d: i32 = reg - ((cycle - 1) % 40);
		if d.abs() <= 1 {
			crt.push('#');
		} else {
			crt.push(' ');
		}
		if cycle % 40 == 0 {
			crt.push('\n');
		}
	};

	for ins in DATA.lines() {
		match &ins[..4] {
			"addx" =>  {
				check_cycle(cycles, reg_x);
				cycles += 1;
				check_cycle(cycles, reg_x);
				cycles += 1;
				reg_x += ins[5..].parse::<i32>().unwrap();

			}
			"noop" => {
				check_cycle(cycles, reg_x);
				cycles += 1;
			}
			_ => unreachable!()
		}
	}
	crt.iter().collect()
}

pub fn print_result() {
	println!("Day10 Part1 Result: {}", part1());
	println!("Day10 Part2 Result:\n{}", part2());
}