use std::{
    collections::HashMap,
    ops::{Add, AddAssign},
};

const DATA: &str = include_str!("../data/day14.txt");

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pos::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

enum Material {
    Air,
    Rock,
    Sand,
}
struct Cave {
    materials: HashMap<Pos, Material>,
    abyss_level: i32,
}

impl Cave {
    fn new() -> Self {
        Self {
            materials: HashMap::new(),
            abyss_level: 0,
        }
    }

    fn get_material(&self, pos: Pos) -> &Material {
        self.materials.get(&pos).unwrap_or(&Material::Air)
    }

    fn place_material(&mut self, pos: Pos, mat: Material) {
        if matches!(mat, Material::Rock) && pos.y > self.abyss_level {
            self.abyss_level = pos.y;
        }
        self.materials.insert(pos, mat);
    }

    fn place_rocks(&mut self, src: Pos, dst: Pos) {
        let sub = (dst.x - src.x, dst.y - src.y);
        let range_x = if sub.0 > 0 { src.x..=dst.x } else { dst.x..=src.x };
        let range_y = if sub.1 > 0 { src.y..=dst.y } else { dst.y..=src.y };

        for i in range_x {
            self.place_material(Pos::new(i, src.y), Material::Rock);
        }
        for i in range_y {
            self.place_material(Pos::new(src.x, i), Material::Rock);
        }
    }
}

struct Simulation {
    cave: Cave,
    sand_spawn_pos: Pos,
    current_sand_pos: Pos,
    sands_rested: u32,
    running: bool,
    bedrock_level: i32,
}

impl Simulation {
    fn new() -> Self {
        Self {
            cave: Cave::new(),
            sand_spawn_pos: Pos::new(500, 0),
            current_sand_pos: Pos::new(500, 0),
            sands_rested: 0,
            running: true,
            bedrock_level: i32::MAX,
        }
    }

    fn step(&mut self) {
        if !self.running {
            return;
        }
        let pos0 = self.current_sand_pos + Pos::new(0, 1);
        let down = self.cave.get_material(pos0);
        if pos0.y < self.bedrock_level {
            if matches!(down, Material::Air) {
                self.cave
                    .place_material(self.current_sand_pos, Material::Air);
                self.current_sand_pos = pos0;
                self.cave
                    .place_material(self.current_sand_pos, Material::Sand);
            } else {
                let pos1 = self.current_sand_pos + Pos::new(-1, 1);
                let pos2 = self.current_sand_pos + Pos::new(1, 1);
                let down_left = self.cave.get_material(pos1);
                let down_right = self.cave.get_material(pos2);
                if matches!(down_left, Material::Air) {
                    self.cave
                        .place_material(self.current_sand_pos, Material::Air);
                    self.current_sand_pos = pos1;
                } else if matches!(down_right, Material::Air) {
                    self.cave
                        .place_material(self.current_sand_pos, Material::Air);
                    self.current_sand_pos = pos2;
                } else {
					self.sands_rested += 1;
                    self.current_sand_pos = self.sand_spawn_pos;
					if matches!(self.cave.get_material(self.sand_spawn_pos), Material::Sand) {
						self.running = false;
						return;
					}
                }
                self.cave
                    .place_material(self.current_sand_pos, Material::Sand);
            }
        } else {
            self.sands_rested += 1;
            self.current_sand_pos = self.sand_spawn_pos;
        }

        if self.current_sand_pos.y > self.cave.abyss_level {
            self.running = false;
        }
    }
}

fn create_simulation() -> Simulation {
    let mut sim = Simulation::new();
    DATA.lines().for_each(|line| {
        let mut it = line
            .split(" -> ")
            .map(|s| {
                let (x, y) = s.split_once(',').unwrap();
                Pos::new(x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
            })
            .peekable();
        while let Some(p) = it.next() {
            if let Some(&n) = it.peek() {
                sim.cave.place_rocks(p, n);
            }
        }
    });
    sim
}

fn part1() -> u32 {
    let mut sim = create_simulation();
    while sim.running {
        sim.step();
    }
    sim.sands_rested
}

fn part2() -> u32 {
    let mut sim = create_simulation();
	sim.cave.abyss_level += 2;
	sim.bedrock_level = sim.cave.abyss_level;
    while sim.running {
        sim.step();
    }
    sim.sands_rested
}

pub fn print_result() {
    println!("Day14 Part1 Result: {}", part1());
    println!("Day14 Part2 Result: {}", part2());
}
