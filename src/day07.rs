#![allow(unused)]
use std::{path::{PathBuf, Path}, collections::{HashSet, HashMap}, ffi::OsString, borrow::Cow, sync::{Arc, Mutex}, rc::Rc, cell::RefCell};

use itertools::Itertools;

#[derive(Debug)]
struct Node<'a> {
	name: &'a str,
	children: Vec<Node<'a>>,
	parent: *mut Node<'a>,
	size: u32
}

impl<'a> Default for Node<'a> {
	fn default() -> Self {
		Self { name: "/", children: Vec::new(), parent: std::ptr::null_mut(), size: 0 }
	}
}

impl<'a> Node<'a> {
	fn get_size(&self) -> u32 {
		self.size + self.children.iter().map(|x| x.get_size()).sum::<u32>()
	}
	
	fn get_dir_sizes(&self) -> Vec<u32> {
		let mut sizes = self.children.iter().flat_map(|x| x.get_dir_sizes()).collect_vec();
		sizes.push(self.get_size());
		sizes
	}
}

const DATA: &str = include_str!("../data/day07.txt");

fn part1_and_2() -> (u32, u32) {
	let mut base = Node::default();
	let mut curr: Option<&mut Node> = Some(&mut base);
	for line in DATA.lines() {	
		let mut c = curr.take();	
		match &line[..4] {
			"$ cd" => match &line[5..] {
				"/" => { curr = Some(&mut base); },
				".." => { curr = Some(unsafe { &mut *c.unwrap().parent }); },
				_ => {
					for n in c.unwrap().children.iter_mut() {
						if n.name == &line[5..] {
							curr = Some(n);
							break;
						}
					}
				}
			}
			"$ ls" => { curr = c; }
			"dir " => {
				let wow = c.unwrap();
				let ptr = wow as *mut Node;
				wow.children.push(Node { name: &line[4..], parent: ptr, ..Default::default() });
				curr = Some(wow);
			}
			_ => {
				let mut it = line.split_whitespace();
				let size = it.next().unwrap().parse::<u32>().unwrap();
				let wow = c.unwrap();

				wow.size += size;
				curr = Some(wow);
			}
		}
	}

	let sizes = base.get_dir_sizes();
	let required = base.get_size() - 40_000_000;
	(sizes.iter().filter(|x| **x <= 100_000).sum(), *sizes.iter().filter(|x| **x >= required).min().unwrap())
}
 
/// WARNING: REQUIRES UNSAFE CODE
pub fn print_result() {
	let r = part1_and_2();
	println!("Day7 Part1 Result: {}", r.0);
	println!("Day7 Part2 Result: {}", r.1);
}