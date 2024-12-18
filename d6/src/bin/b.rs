use std::io;

fn print_grid(g: &Vec<Vec<bool>>, px: usize, py:usize, dir: i32) {
  for (y, row) in g.iter().enumerate() {
    for (x, cell) in row.iter().enumerate() {
      let symbol = if x == px && y == py {
        match dir {
          0 => '^',
          1 => '>',
          2 => 'v',
          3 => '<',
          _ => panic!("Unknown direction")
        }
      } else if *cell {
        '#'
      } else {
        '.'
      };
      print!("{}", symbol);
    }
    println!();
  }
}

fn test(g: &Vec<Vec<bool>>, startx: usize, starty: usize, obstaclex: usize, obstacley: usize) -> bool {
  let width = g[0].len();
  let height = g.len();
  let mut visited: Vec<Vec<u8>> = vec![];
  for _ in 0..height {
    let row = vec![0 as u8; width];
    visited.push(row);
  }
  let mut x = startx;
  let mut y = starty;
  let mut dir = 0;
  loop {
    if visited[y][x] & (1 << dir) != 0 {
      // A loop
      return true;
    }
    visited[y][x] |= 1 << dir;

    loop {
      let mut aheadx = x;
      let mut aheady = y;
      match dir {
        0 => {
          if y == 0 {
            break;
          }
          aheady -= 1;
        },
        1 => {
          if x == width - 1 {
            break;
          }
          aheadx += 1;
        },
        2 => {
          if y == height - 1 {
            break;
          }
          aheady += 1;
        },
        3 => {
          if x == 0 {
            break;
          }
          aheadx -= 1;
        },
        _ => panic!("Bad direction")
      }
      if g[aheady][aheadx] || (aheadx == obstaclex && aheady == obstacley) {
        dir = (dir + 1) % 4;
        visited[y][x] |= 1 << dir;
      } else {
        break;
      }
    }
    match dir {
      0 => {
        if y == 0 {
          break;
        }
        y -= 1;
      },
      1 => {
        if x == width - 1 {
          break;
        }
        x += 1;
      },
      2 => {
        if y == height - 1 {
          break;
        }
        y += 1;
      },
      3 => {
        if x == 0 {
          break;
        }
        x -= 1;
      },
      _ => panic!("Bad direction")
    }
  }
  return false;
}

fn main() {
  let mut g: Vec<Vec<bool>> = vec![];
  let mut x = 0;
  let mut y = 0;
  let mut buffer = String::new();
  while io::stdin().read_line(&mut buffer).is_ok() {
    if buffer.trim().is_empty() {
      break;
    }
    let mut row = vec![];
    for cell in buffer.trim().chars() {
      match cell {
        '.' => row.push(false),
        '#' => row.push(true),
        '^' => {
          row.push(false);
          x = row.len() - 1;
          y = g.len();
        },
        c => panic!("Unknown char {c}")
      }
    }
    g.push(row);
    buffer.clear();
  }
  let start_x = x;
  let start_y = y;
  // print_grid(&g, x, y, dir);

  let width = g[0].len();
  let height = g.len();

  let mut loopcount = 0;
  for obstaclex in 0..width {
    for obstacley in 0..height {
      if !g[obstacley][obstaclex] && !(start_x == obstaclex && start_y == obstacley) {
        if test(&g, start_x, start_y, obstaclex, obstacley) {
          loopcount += 1;
        }
      }
    }
  }

  println!("{}", loopcount);
}
