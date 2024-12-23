use std::{collections::HashSet, io};

fn print_grid(g: &Vec<Vec<char>>) {
  for row in g {
    for cell in row {
      print!("{}", cell);
    }
    println!();
  }
}

fn region_of(x: usize, y:usize, g: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) -> HashSet<(usize, usize)> {
  let mut ret = HashSet::new();
  if !visited[y][x] {
    visited[y][x] = true;
    ret.insert((x, y));
    let t = g[y][x];
    if x > 0 && g[y][x-1] == t {
      for cell in region_of(x-1, y, g, visited).iter() {
        ret.insert(*cell);
      }
    }
    if x < g[y].len() - 1 && g[y][x+1] == t {
      for cell in region_of(x+1, y, g, visited).iter() {
        ret.insert(*cell);
      }
    }
    if y > 0 && g[y-1][x] == t {
      for cell in region_of(x, y-1, g, visited).iter() {
        ret.insert(*cell);
      }
    }
    if y < g.len() - 1 && g[y + 1][x] == t {
      for cell in region_of(x, y+1, g, visited).iter() {
        ret.insert(*cell);
      }
    }
  }

  ret
}

fn region_borders(x: usize, y:usize, g: &Vec<Vec<char>>) -> u32 {
  let mut count = 0;
  let t = g[y][x];
  if x == 0 || g[y][x-1] != t {
    count += 1;
  }
  if x >= g[y].len() - 1 || g[y][x+1] != t {
    count += 1;
  }
  if y == 0 || g[y-1][x] != t {
    count += 1;
  }
  if y >= g.len() - 1 || g[y + 1][x] != t {
    count += 1;
  }

  count
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
        let cells = region_of(x, y, &g, &mut visited);
        let perimeter = cells.iter()
          .map(|(x, y)| region_borders(*x, *y, &g))
          .reduce(|acc, p| acc + p).unwrap();
        let area = cells.len();
        cost += perimeter * area as u32;
      }
    }
  }

  println!("{}", cost);
}