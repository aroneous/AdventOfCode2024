use std::{io, vec};

#[derive(Debug, Clone, Copy)]
enum Cell {
  Empty,
  Wall,
  Box,
  Robot,
}

#[derive(Debug, Clone)]
enum Dir {
  Up,
  Down,
  Left,
  Right,
}

use Cell::*;
use Dir::*;

fn print_grid(g: &Vec<Vec<Cell>>) {
  for row in g {
    for cell in row {
      let c = match cell {
        Empty => '.',
        Wall => '#',
        Box => 'O',
        Robot => '@',
      };
      print!("{}", c);
    }
    println!();
  }
}

fn print_moves(moves: &Vec<Dir>) {
  for dir in moves {
    let c = match dir {
      Up => '^',
      Down => 'v',
      Left => '<',
      Right => '>',
    };
    print!("{}", c);
  }
  println!();
}

fn next(x: usize, y: usize, dir: &Dir) -> (usize, usize) {
  match dir {
    Up => (x, y - 1),
    Down => (x, y + 1),
    Left => (x - 1, y),
    Right => (x + 1, y),
  }
}

fn try_move(g: &mut Vec<Vec<Cell>>, x: usize, y: usize, dir: &Dir) -> Option<(usize, usize)> {
  match g[y][x] {
    Wall => None,
    Empty => Some(next(x, y, dir)),
    _ => {
      let (xp, yp) = next(x, y, dir);
      if let Some(_) = try_move(g, xp, yp, dir) {
        g[yp][xp] = g[y][x];
        g[y][x] = Empty;
        Some((xp, yp))
      } else {
        None
      }
    }
  }
}

fn main() {
  let mut moves_phase = false;

  let mut g = vec![];
  let mut moves = vec![];
  let mut x = 0;
  let mut y = 0;
  for r in io::stdin().lines() {
    let line = r.unwrap();

    if !moves_phase {
      if line.is_empty() {
        moves_phase = true;
        continue;
      }
      // Read grid
      let mut row = vec![];
      for c in line.chars() {
        let cell = match c {
          '.' => Empty,
          '#' => Wall,
          'O' => Box,
          '@' => Robot,
          _ => panic!("Unknown character {c}")
        };
        if let Robot = cell {
          x = row.len();
          y = g.len();
        }
        row.push(cell);
      }
      g.push(row);
    } else {
      // Read moves
      for c in line.chars() {
        let dir = match c {
          '^' => Up,
          'v' => Down,
          '<' => Left,
          '>' => Right,
          _ => panic!("Unknown move {c}")
        };
        moves.push(dir);
      }
    }
  }

  print_grid(&g);
  println!();
  println!("Robot at ({x}, {y})");
  println!();
  print_moves(&moves);

  for dir in moves {
    if let Some((xp, yp)) = try_move(&mut g, x, y, &dir) {
      x = xp;
      y = yp;
    }
  }

  print_grid(&g);

  let mut answer = 0;
  for (y, row) in g.iter().enumerate() {
    for (x, cell) in row.iter().enumerate() {
      if let Box = cell {
        answer += 100*y + x;
      }
    }
  }

  println!("{answer}");
}
