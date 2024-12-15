use std::io;

// static VAL: Vec<char> = vec!['X', 'M', 'A', 'S'];
const VAL: &[char] = &['X', 'M', 'A', 'S'];

fn check(g: &Vec<Vec<char>>, x: usize, y: usize, dx: isize, dy: isize) -> bool {
  let height = g.len();
  let width = g[0].len();
  let mut state = 0;
  let mut cx = x as isize;
  let mut cy = y as isize;
  while cx >= 0 && cx < width as isize && cy >= 0 && cy < height as isize {
    if VAL[state] == g[cy as usize][cx as usize] {
      state += 1;
      if state >= VAL.len() {
        return true;
      }
    } else {
      return false;
    }
    cx = cx as isize + dx;
    cy = cy as isize + dy;
  }
  return false;
}

fn checkcell(g: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
  let mut count = 0;
  if check(g, x, y, 1, 0) { count += 1 };
  if check(g, x, y, 0, 1) { count += 1 };
  if check(g, x, y, -1, 0) { count += 1 };
  if check(g, x, y, 0, -1) { count += 1 };
  if check(g, x, y, 1, 1) { count += 1 };
  if check(g, x, y, -1, -1) { count += 1 };
  if check(g, x, y, -1, 1) { count += 1 };
  if check(g, x, y, 1, -1) { count += 1 };

  return count;
}

fn main() {
  let mut buffer = String::new();
  let mut g: Vec<Vec<char>> = vec![];
  while io::stdin().read_line(&mut buffer).is_ok() {
    if buffer.is_empty() {
      break;
    }
    g.push(buffer.trim().chars().collect());
    buffer.clear();
  }

  let mut count = 0;
  for x in 0..g[0].len() {
    for y in 0..g.len() {
      count += checkcell(&g, x, y);
    }
  }

  println!("{}", count);
}
