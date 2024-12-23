use std::io;

struct Region {
  area: u32,
  corners: u32,
}

impl Region {
  fn empty() -> Region {
    Region { area: 0, corners: 0 }
  }

  // fn new(area: u32, corners: u32) -> Region {
  //   Region { area, corners }
  // }

  fn add(&mut self, other: &Region) {
    self.area += other.area;
    self.corners += other.corners;
  }
}

fn print_grid(g: &Vec<Vec<char>>) {
  for row in g {
    for cell in row {
      print!("{}", cell);
    }
    println!();
  }
}

fn region_of(x: usize, y:usize, g: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) -> Region {
  let mut ret = Region::empty();
  if !visited[y][x] {
    visited[y][x] = true;
    ret.area += 1;
    let t = g[y][x];
    let mut left = false;
    let mut right = false;
    let mut up = false;
    let mut down = false;
    if x > 0 && g[y][x-1] == t {
      ret.add(&region_of(x-1, y, g, visited));
    } else {
      left = true;
    }
    if x < g[y].len() - 1 && g[y][x+1] == t {
      ret.add(&region_of(x+1, y, g, visited));
    } else {
      right = true;
    }
    if y > 0 && g[y-1][x] == t {
      ret.add(&region_of(x, y-1, g, visited));
    } else {
      up = true;
    }
    if y < g.len() - 1 && g[y + 1][x] == t {
      ret.add(&region_of(x, y+1, g, visited));
    } else {
      down = true;
    }
    if x == 0 {
      left = true;
    }
    if y == 0 {
      up = true;
    }
    if x == g[y].len() - 1 {
      right = true;
    }
    if y == g.len() - 1 {
      down = true;
    }

    let mut corners = 0;
    // Inside corners
    if up && right {
      corners += 1;
    }
    if right && down {
      corners += 1;
    }
    if down && left {
      corners += 1;
    }
    if left && up {
      corners += 1;
    }
    // Outside corners
    if !up && !right {
      if x < g[y].len() - 1 && y > 0 && g[y-1][x+1] != t {
        corners += 1;
      }
    }
    if !right && !down {
      if x < g[y].len() - 1 && y < g.len() - 1 && g[y+1][x+1] != t {
        corners += 1;
      }
    }
    if !down && !left {
      if x > 0 && y < g.len() - 1 && g[y+1][x-1] != t {
        corners += 1;
      }
    }
    if !left && !up {
      if x > 0 && y > 0 && g[y-1][x-1] != t {
        corners += 1;
      }
    }
    ret.corners += corners;
  }

  ret
}

fn main() {
  let mut g: Vec<Vec<char>> = vec![];
  let mut buffer = String::new();
  while io::stdin().read_line(&mut buffer).is_ok() {
    let line = buffer.trim();
    if line.is_empty() {
      break;
    }
    let mut row: Vec<char> = vec![];
    for cell in line.chars() {
      row.push(cell);
    }
    g.push(row);
    buffer.clear();
  }
  // print_grid(&g);

  let width = g[0].len();
  let height = g.len();

  let mut visited = vec![];
  for _ in 0..height {
    let row = vec![false; width];
    visited.push(row);
  }

  let mut cost = 0;
  for x in 0..width {
    for y in 0..height {
      if !visited[y][x] {
        let region = region_of(x, y, &g, &mut visited);
        cost += region.corners * region.area;
      }
    }
  }

  println!("{}", cost);
}