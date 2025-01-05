use std::{cmp::min, io, u32, vec};

#[derive(Debug, Clone, Copy)]
enum Cell {
  Empty,
  Wall,
}

#[derive(Debug, Clone, Copy)]
enum Dir {
  North,
  East,
  South,
  West
}

impl Dir {
  fn right(&self) -> Dir {
    match self {
      North => East,
      East => South,
      South => West,
      West => North,
    }
  }

  fn left(&self) -> Dir {
    match self {
      North => West,
      West => South,
      South => East,
      East => North,
    }
  }

  fn behind(&self) -> Dir {
    match self {
      North => South,
      East => West,
      South => North,
      West => East,
    }
  }
}

#[derive(Debug, Clone, Copy)]
struct Cost {
  north: u32,
  east: u32,
  south: u32,
  west: u32,
}

impl Cost {
  fn new() -> Cost {
    Cost { north: u32::MAX, east: u32::MAX, south: u32::MAX, west: u32::MAX }
  }

  fn in_dir(&self, dir: &Dir) -> u32 {
    match dir {
      North => self.north,
      East => self.east,
      South => self.south,
      West => self.west,
    }
  }

  fn cost_ref(&mut self, dir: &Dir) -> &mut u32 {
    match dir {
      North => &mut self.north,
      East => &mut self.east,
      South => &mut self.south,
      West => &mut self.west,
    }
  }

  fn set_if_better(&mut self, dir: Dir, cost: u32) -> bool {
    let curr = self.cost_ref(&dir);
    if cost < *curr {
      *curr = cost;
      true
    } else {
      false
    }
  }

  fn min(&self) -> u32 {
    let mut val = min(self.north, self.east);
    val = min(val, self.south);
    val = min(val, self.west);
    val
  }
}

use Cell::*;
use Dir::*;

fn print_grid(g: &Vec<Vec<Cell>>) {
  for row in g {
    for cell in row {
      let c = match cell {
        Empty => '.',
        Wall => '#',
      };
      print!("{}", c);
    }
    println!();
  }
}

fn neighbor_in_dir(g: &Vec<Vec<Cell>>, x: usize, y: usize, dir: Dir) -> Option<(usize, usize)> {
  let (xp, yp) = match dir {
    North => (x, y - 1),
    East => (x + 1, y),
    South => (x, y + 1),
    West => (x - 1, y)
  };
  if let Empty = g[yp][xp] {
    Some((xp, yp))
  } else {
    None
  }
}

fn handle(g: &Vec<Vec<Cell>>, cost: &mut Vec<Vec<Cost>>, x: usize, y: usize, dir: Dir, curr_cost: u32) {
  let c = &mut cost[y][x];
  let mut improved;
  improved = c.set_if_better(dir, curr_cost);
  improved = c.set_if_better(dir.right(),  curr_cost + 1000) || improved;
  improved = c.set_if_better(dir.left(),  curr_cost + 1000) || improved;
  improved = c.set_if_better(dir.behind(),  curr_cost + 2000) || improved;

  let c2 = cost[y][x];
  if improved {
    if let Some((px, py)) = neighbor_in_dir(g, x, y, dir) {
      handle(g, cost, px, py, dir, c2.in_dir(&dir) + 1);
    }
    let left_dir = dir.left();
    if let Some((px, py)) = neighbor_in_dir(g, x, y, left_dir) {
      handle(g, cost, px, py, left_dir, c2.in_dir(&left_dir) + 1);
    }
    let right_dir = dir.right();
    if let Some((px, py)) = neighbor_in_dir(g, x, y, right_dir) {
      handle(g, cost, px, py, right_dir, c2.in_dir(&right_dir) + 1);
    }
    let behind_dir = dir.behind();
    if let Some((px, py)) = neighbor_in_dir(g, x, y, behind_dir) {
      handle(g, cost, px, py, behind_dir, c2.in_dir(&behind_dir) + 1);
    }
  }
}

fn main() {
  let mut g = vec![];
  let mut sx = 0;
  let mut sy = 0;
  let mut ex = 0;
  let mut ey = 0;
  for r in io::stdin().lines() {
    let line = r.unwrap();

    let mut row = vec![];
    for c in line.chars() {
      let cell = match c {
        '.' => Empty,
        '#' => Wall,
        'S' => {
          sy = g.len();
          sx = row.len();
          Empty
        },
        'E' => {
          ey = g.len();
          ex = row.len();
          Empty
        },
        _ => panic!("Unknown character {c}")
      };
      row.push(cell);
    }
    g.push(row);
  }

  print_grid(&g);

  let height = g.len();
  let width = g[0].len();

  let mut cost = vec![];
  for _ in 0..height {
    cost.push(vec![Cost::new(); width]);
  }

  handle(&g, &mut cost, sx, sy, East, 0);
  let answer = cost[ey][ex].min();
  println!("{}", answer);
}
