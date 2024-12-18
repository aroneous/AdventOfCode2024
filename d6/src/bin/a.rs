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

fn main() {
  let mut g: Vec<Vec<bool>> = vec![];
  let mut visited: Vec<Vec<bool>> = vec![];
  let mut x = 0;
  let mut y = 0;
  let mut dir = 0;
  let mut buffer = String::new();
  while io::stdin().read_line(&mut buffer).is_ok() {
    if buffer.trim().is_empty() {
      break;
    }
    let mut row = vec![];
    let mut vrow = vec![];
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
      vrow.push(false);
    }
    g.push(row);
    visited.push(vrow);
    buffer.clear();
  }
  // print_grid(&g, x, y, dir);

  let width = g[0].len();
  let height = g.len();
  let mut count = 0;
  loop {
    if !visited[y][x] {
      count += 1;
      visited[y][x] = true;
    }

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
    if g[aheady][aheadx] {
      dir = (dir + 1) % 4;
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

  println!("{}", count);
}
