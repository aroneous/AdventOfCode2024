use std::{collections::HashSet, io};

fn print_grid(g: &Vec<Vec<u8>>) {
  for row in g {
    for cell in row {
      print!("{}", cell);
    }
    println!();
  }
}

fn next_step(g: &Vec<Vec<u8>>, curr: HashSet<(usize, usize)>, next_level: u8) -> HashSet<(usize, usize)> {
  let mut ret = HashSet::new();

  for (xref, yref) in curr.iter() {
    let x = *xref;
    let y = *yref;
    if x > 0 && g[y][x-1] == next_level {
      ret.insert((x-1, y));
    }
    if x < g[y].len() - 1 && g[y][x+1] == next_level {
      ret.insert((x+1, y));
    }
    if y > 0 && g[y-1][x] == next_level {
      ret.insert((x, y-1));
    }
    if y < g.len() - 1 && g[y + 1][x] == next_level {
      ret.insert((x, y+1));
    }
  }
  ret
}

fn main() {
  let mut g: Vec<Vec<u8>> = vec![];
  let mut buffer = String::new();
  while io::stdin().read_line(&mut buffer).is_ok() {
    let line = buffer.trim();
    if line.is_empty() {
      break;
    }
    let mut row = vec![];
    for cell in line.bytes() {
      let val = cell - 48;
      row.push(val);
    }
    g.push(row);
    buffer.clear();
  }
  // print_grid(&g);

  let mut sum = 0;
  
  for (y, row) in g.iter().enumerate() {
    for (x, cell) in row.iter().enumerate() {
      if *cell == 0 {
        let mut curr = HashSet::new();
        curr.insert((x, y));
        for next_level in 1..=9 {
          let next = next_step(&g, curr, next_level);
          curr = next;
        }
        sum += curr.len();
      }
    }
  }

  println!("{}", sum);
}