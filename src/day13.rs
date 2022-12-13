use std::{str::Chars, iter::Peekable, cmp::Ordering};

use itertools::Itertools;

const DATA: &str = include_str!("../data/day13.txt");

#[derive(Debug, PartialEq, Eq, Clone)]
enum Node {
	Int(u32),
	List(Vec<Node>)
}

fn parse_node(first: char, s: &mut Peekable<Chars>) -> Node {
	match first {
		'[' => {
			let mut items = Vec::new();
			while let Some(c) = s.next() {
				if c == ']' {
					break;
				}
				if c == ',' {
					continue;
				}
				items.push(parse_node(c, s));
			}
			Node::List(items)
		},
		_ => {
			let mut a = first.to_string();
			while let Some(&c) = s.peek() {
				if c != '[' && c != ']' && c != ',' {
					a.push(s.next().unwrap());
				} else {
					break;
				}
			}
			Node::Int(a.parse::<u32>().unwrap())
		}
	}
}

fn compare_pairs(pair: (&Node, &Node)) -> Ordering {
	match pair {
		(Node::List(a), Node::List(b)) => {
			let mut it = b.iter();
			for i in a.iter() {
				if let Some(j) = it.next() {
					let ord = compare_pairs((i, j));
					if ord != Ordering::Equal {
						return ord;
					}
				}
			}
			a.len().cmp(&b.len())
		}
		(Node::List(_), Node::Int(b)) => compare_pairs((pair.0, &Node::List(vec![Node::Int(*b)]))),
		(Node::Int(a), Node::List(_)) => compare_pairs((&Node::List(vec![Node::Int(*a)]), pair.1)),
		(Node::Int(a), Node::Int(b)) => a.cmp(b)
	}
}

fn part1() -> usize {
	let data = DATA.replace('\r', "");
	let pairs = data.split("\n\n").map(|x| {
		let (left, right) = x.split_once('\n').unwrap();
		let (mut lit, mut rit) = (left.chars().peekable(), right.chars().peekable());
		(parse_node(lit.next().unwrap(), &mut lit), parse_node(rit.next().unwrap(), &mut rit))
	}).collect_vec();
	pairs.iter().enumerate().filter_map(|(i, (left, right))| if compare_pairs((left, right)) == Ordering::Less { Some(i + 1) } else { None }).sum()
}

fn part2() -> usize {
	let data = DATA.replace('\r', "");
	let mut pairs = data.split('\n')
		.filter(|x| !x.is_empty())
		.map(|x| {
			let mut it = x.chars().peekable();
			parse_node(it.next().unwrap(), &mut it)
	}).collect_vec();
	let div6 = Node::List(vec![Node::List(vec![Node::Int(6)])]);
	let div3 = Node::List(vec![Node::List(vec![Node::Int(2)])]);
	pairs.push(div6.clone());
	pairs.push(div3.clone());
	pairs.iter()
		.sorted_by(|&left, &right| compare_pairs((left, right)))
		.enumerate()
		.filter(|&(_, node)| node.eq(&div6) || node.eq(&div3))
		.map(|(i, _)| i + 1)
		.product()
}

pub fn print_result() {
	println!("Day13 Part1 Result: {}", part1());
	println!("Day13 Part2 Result: {}", part2());
}