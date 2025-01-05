use std::{cmp::min, collections::HashSet, io, u32, vec};

#[derive(Debug, Clone, Copy)]
enum Cell {
  Empty,
  Wall,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
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

#[derive(Debug, Clone)]
struct CostAndPrev {
  cost: u32,
  prev: HashSet<(usize, usize, Dir)>,
}

impl CostAndPrev {
  fn new() -> CostAndPrev {
    CostAndPrev { cost: u32::MAX, prev: HashSet::new() }
  }
}

#[derive(Debug, Clone)]
struct Cost {
  north: CostAndPrev,
  east: CostAndPrev,
  south: CostAndPrev,
  west: CostAndPrev,
}

impl Cost {
  fn new() -> Cost {
    Cost { north: CostAndPrev::new(), east: CostAndPrev::new(), south: CostAndPrev::new(), west: CostAndPrev::new() }
  }

  fn in_dir(&self, dir: &Dir) -> u32 {
    match dir {
      North => self.north.cost,
      East => self.east.cost,
      South => self.south.cost,
      West => self.west.cost,
    }
  }

  fn cost_ref(&mut self, dir: &Dir) -> &mut CostAndPrev {
    match dir {
      North => &mut self.north,
      East => &mut self.east,
      South => &mut self.south,
      West => &mut self.west,
    }
  }

  fn cost_ref_imm(&self, dir: &Dir) -> &CostAndPrev {
    match dir {
      North => &self.north,
      East => &self.east,
      South => &self.south,
      West => &self.west,
    }
  }

  fn set_if_better(&mut self, dir: Dir, cost: u32, prev: Option<(usize, usize, Dir)>) -> bool {
    let curr = self.cost_ref(&dir);
    if cost < curr.cost {
      curr.cost = cost;
      curr.prev.clear();
      if let Some(prev_cell) = prev {
        curr.prev.insert(prev_cell);
      }
      true
    } else if cost == curr.cost {
      if let Some(prev_cell) = prev {
        curr.prev.insert(prev_cell);
      }
      false
    } else {
      false
    }
  }

  fn min(&self) -> u32 {
    let mut val = min(self.north.cost, self.east.cost);
    val = min(val, self.south.cost);
    val = min(val, self.west.cost);
    val
  }

  fn prev_for(&self, dir: Dir) -> &HashSet<(usize, usize, Dir)> {
    &self.cost_ref_imm(&dir).prev
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

fn handle(g: &Vec<Vec<Cell>>, cost: &mut Vec<Vec<Cost>>, x: usize, y: usize, dir: Dir, curr_cost: u32,
  prev: Option<(usize, usize, Dir)>

    // best_route_cells: &mut HashSet<(usize, usize)>, path: &mut Vec<(usize, usize)>, bestcost: &mut u32,
    // ex: usize, ey: usize
  ) {
  let c = &mut cost[y][x];
  let mut improved;
  improved = c.set_if_better(dir, curr_cost, prev);
  improved = c.set_if_better(dir.right(),  curr_cost + 1000, prev) || improved;
  improved = c.set_if_better(dir.left(),  curr_cost + 1000, prev) || improved;
  improved = c.set_if_better(dir.behind(),  curr_cost + 2000, prev) || improved;

  let c2 = &cost[y][x];
  if improved {
    // path.push((x, y));
    // let ccost = c2.min();
    let ccf = c2.in_dir(&dir) + 1;
    let left_dir = dir.left();
    let ccl = c2.in_dir(&left_dir) + 1;
    let right_dir = dir.right();
    let ccr = c2.in_dir(&right_dir) + 1;
    let behind_dir = dir.behind();
    let ccb = c2.in_dir(&behind_dir) + 1;
    // if x == ex && y == ey {
    //   if ccost < *bestcost {
    //     *bestcost = ccost;
    //     best_route_cells.clear();
    //   }
    //   if ccost == *bestcost {
    //     for cell in & *path {
    //       best_route_cells.insert(*cell);
    //     }
    //   }
    // }

    if let Some((px, py)) = neighbor_in_dir(g, x, y, dir) {
      handle(g, cost, px, py, dir, ccf, Some((x, y, dir)));
    }
    if let Some((px, py)) = neighbor_in_dir(g, x, y, left_dir) {
      handle(g, cost, px, py, left_dir, ccl, Some((x, y, left_dir)));
    }
    if let Some((px, py)) = neighbor_in_dir(g, x, y, right_dir) {
      handle(g, cost, px, py, right_dir, ccr, Some((x, y, right_dir)));
    }
    if let Some((px, py)) = neighbor_in_dir(g, x, y, behind_dir) {
      handle(g, cost, px, py, behind_dir, ccb, Some((x, y, behind_dir)));
    }
    // path.pop();
  }
}

fn count_pred(cost: &Vec<Vec<Cost>>, path_cells: &mut HashSet<(usize, usize)>, prevs: &HashSet<(usize, usize, Dir)>) {
  for (px, py, pdir) in prevs {
    path_cells.insert((*px, *py));
    count_pred(cost, path_cells, &cost[*py][*px].cost_ref_imm(&pdir).prev);
  }
}

fn main() {
  let mut g = vec![];
  let mut sx = 0;
  let mut sy = 0;
  let mut ex = 0;
  let mut ey = 0;

  // let mut best_route_cells = HashSet::<(usize, usize)>::new();
  // let mut path = vec![];
  // let mut bestcost = u32::MAX;

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

  handle(&g, &mut cost, sx, sy, East, 0, None);

  let mut path_cells = HashSet::new();
  let ecost = &cost[ey][ex];
  let emin = ecost.min();
  path_cells.insert((ex, ey));
  if ecost.north.cost == emin {
    let prevs = ecost.prev_for(North);
    count_pred(&cost, &mut path_cells, &prevs);
  }
  println!("{}", path_cells.len());
}
