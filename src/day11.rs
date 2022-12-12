use std::collections::HashMap;

use itertools::Itertools;

const DATA: &str = include_str!("../data/day11.txt");

type MonkeyList = Vec<(Box<dyn Fn(u64) -> u64>, u64, Box<dyn Fn(bool) -> usize>, Vec<u64>)>;

fn get_monkeys() -> MonkeyList {
	let data = DATA.replace('\r', "");
	data.split("\n\n")
		.map(|sec| {
			let mut lines = sec.lines();
			let items = lines.nth(1).unwrap()[18..].split(", ")
				.map(|x| x.parse::<u64>().unwrap()).collect_vec();
			let op = &lines.next().unwrap()[19..];
			let o: Box<dyn Fn(u64) -> u64> = match op {
				"old * old" => Box::new(|a| a * a),
				"old + old" => Box::new(|a| a + a),
				_ => {
					let s = op[6..].parse::<u64>().unwrap();
					match &op[..5] {
						"old *" => Box::new(move |a| a * s),
						"old +" => Box::new(move |a| a + s),
						_ => unreachable!()
					}
				}
			};
			let div = lines.next().unwrap()[21..].parse::<u64>().unwrap();
			let t = lines.next().unwrap()[29..].parse::<usize>().unwrap();
			let f = lines.next().unwrap()[30..].parse::<usize>().unwrap();
			let v: Box<dyn Fn(bool) -> usize> = Box::new(move |x| if x { t } else { f });
			(o, div, v, items)
		}).collect_vec()
}

fn inspect_items(mut monkeys: MonkeyList, rounds: u64, prediction: impl Fn(u64) -> u64) -> usize {
	let mut i = 0;
	let mut round = 0;
	let mut inspected = Vec::new();
	while round < rounds {
		while i < monkeys.len() {
			let monkey = &mut monkeys[i];
			let op = &monkey.0;
			let check = &monkey.2;
			if let Some(ins) = inspected.get_mut(i) {
				*ins += monkey.3.len();
			} else {
				inspected.insert(i, monkey.3.len());
			}

			let mut add: HashMap<usize, Vec<u64>> = HashMap::new(); 

			for item in monkey.3.iter() {
				let worry = prediction(op(*item));
				add.entry(check(worry % monkey.1 == 0)).and_modify(|x| x.push(worry)).or_insert_with(|| vec![worry]);
			}
			monkey.3.clear();
			for (mid, item) in add.iter_mut() {
				monkeys[*mid].3.append(item);
			}
			i += 1;
		}
		i = 0;
		round += 1;
	}
	inspected.iter().sorted().rev().take(2).product()
}

fn part1() -> usize {
	inspect_items(get_monkeys(), 20, |x| (x as f64 / 3.0).floor() as u64)
}

fn part2() -> usize {//boklama zamani geldi
	let monkeys = get_monkeys();
	let div_max = monkeys.iter().map(|x| x.1).product::<u64>();
	inspect_items(monkeys, 10000, |x| x % div_max)
}

pub fn print_result() {
	println!("Day11 Part1 Result: {}", part1());
	println!("Day11 Part2 Result: {}", part2());
}