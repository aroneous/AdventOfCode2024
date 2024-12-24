use std::{io, vec};

#[derive(Debug, Clone, Copy)]
enum Cell {
  Empty,
  Wall,
  BoxL,
  BoxR,
  Robot,
}

#[derive(Debug, Clone)]
enum Dir {
  Up,
  Down,
  Left,
  Right,
}

impl Dir {
  fn is_vertical(&self) -> bool {
    match self {
      Up => true,
      Down => true,
      _ => false
    }
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
        BoxL => '[',
        BoxR => ']',
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

fn can_move(g: &mut Vec<Vec<Cell>>, x: usize, y: usize, dir: &Dir, other_half: bool) -> bool {
  match g[y][x] {
    Wall => false,
    Empty => true,
    Robot => {
      let (xp, yp) = next(x, y, dir);
      if can_move(g, xp, yp, dir, false) {
        true
      } else {
        false
      }
    },
    BoxL => {
      let (xp, yp) = next(x, y, dir);
      let paired;
      if !other_half && dir.is_vertical() {
        let (xr, yr) = next(x, y, &Right);
        // let (xp2, yp2) = next(xr, yr, dir);
        paired = can_move(g, xr, yr, dir, true);
      } else {
        paired = true;
      }
      if can_move(g, xp, yp, dir, false) && paired {
        true
      } else {
        false
      }
    },
    BoxR => {
      let (xp, yp) = next(x, y, dir);
      let paired;
      if !other_half && dir.is_vertical() {
        let (xl, yl) = next(x, y, &Left);
        // let (xp2, yp2) = next(xl, yl, dir);
        paired = can_move(g, xl, yl, dir, true);
      } else {
        paired = true;
      }
      if can_move(g, xp, yp, dir, false) && paired {
        true
      } else {
        false
      }
    }
  }
}


fn do_move(g: &mut Vec<Vec<Cell>>, x: usize, y: usize, dir: &Dir, other_half: bool) -> (usize, usize) {
  match g[y][x] {
    Wall => panic!("Can't move a wall"),
    Empty => next(x, y, dir),
    Robot => {
      let (xp, yp) = next(x, y, dir);
      do_move(g, xp, yp, dir, false);
      g[yp][xp] = g[y][x];
      g[y][x] = Empty;
      (xp, yp)
    },
    BoxL => {
      let (xp, yp) = next(x, y, dir);
      if !other_half && dir.is_vertical() {
        let (xr, yr) = next(x, y, &Right);
        do_move(g, xr, yr, dir, true);
      }

      do_move(g, xp, yp, dir, false);
      g[yp][xp] = g[y][x];
      g[y][x] = Empty;
      (xp, yp)

    },
    BoxR => {
      let (xp, yp) = next(x, y, dir);
      if !other_half && dir.is_vertical() {
        let (xl, yl) = next(x, y, &Left);
        do_move(g, xl, yl, dir, true);
      }

      do_move(g, xp, yp, dir, false);
      g[yp][xp] = g[y][x];
      g[y][x] = Empty;
      (xp, yp)
    }
  }
}

fn try_move(g: &mut Vec<Vec<Cell>>, x: usize, y: usize, dir: &Dir) -> Option<(usize, usize)> {
  if can_move(g, x, y, dir, false) {
    Some(do_move(g, x, y, dir, false))
  } else {
    None
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
        let (cell, cell2) = match c {
          '.' => (Empty, Empty),
          '#' => (Wall, Wall),
          'O' => (BoxL, BoxR),
          '@' => (Robot, Empty),
          _ => panic!("Unknown character {c}")
        };
        if let Robot = cell {
          x = row.len();
          y = g.len();
        }
        row.push(cell);
        row.push(cell2);
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
      // print_grid(&g);
    }
  }

  print_grid(&g);

  let mut answer = 0;
  for (y, row) in g.iter().enumerate() {
    for (x, cell) in row.iter().enumerate() {
      if let BoxL = cell {
        answer += 100*y + x;
      }
    }
  }

  println!("{answer}");
}
