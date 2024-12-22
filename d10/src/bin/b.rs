use std::io;

fn print_grid(g: &Vec<Vec<u8>>) {
  for row in g {
    for cell in row {
      print!("{}", cell);
    }
    println!();
  }
}

fn paths_from(g: &Vec<Vec<u8>>, x: usize, y: usize, next_level: u8) -> u64 {
  if next_level > 9 {
    return 1;
  }

  let mut sum = 0;

  if x > 0 && g[y][x-1] == next_level {
    sum += paths_from(g, x-1, y, next_level + 1);
  }
  if x < g[y].len() - 1 && g[y][x+1] == next_level {
    sum += paths_from(g, x+1, y, next_level + 1);
  }
  if y > 0 && g[y-1][x] == next_level {
    sum += paths_from(g, x, y-1, next_level + 1);
  }
  if y < g.len() - 1 && g[y + 1][x] == next_level {
    sum += paths_from(g, x, y+1, next_level + 1);
  }

  return sum;
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
        sum += paths_from(&g, x, y, 1);
      }
    }
  }

  println!("{}", sum);
}