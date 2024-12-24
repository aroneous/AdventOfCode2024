use std::io;
use regex::Regex;

enum ParseState {
  A,
  B,
  Prize,
  Blank
}

#[derive(Debug)]
struct Coords {
  x: u64,
  y: u64
}

impl  Coords {
  fn new() -> Coords {
    Coords { x: 0, y: 0 }
  }
}

#[derive(Debug)]
struct Case {
  a: Coords,
  b: Coords,
  prize: Coords
}

impl Case {
  fn new() -> Case {
    Case { a: Coords::new(), b: Coords::new(), prize: Coords::new()}
  }
}

use ParseState::*;

fn parse_line(line: &str) -> Coords {
  let re = Regex::new(r"X(?:\+|=)(\d+), Y(?:\+|=)(\d+)").unwrap();
  let (_, [xstr, ystr]) = re.captures(line).unwrap().extract();
  let x: u64 = xstr.parse().unwrap();
  let y: u64 = ystr.parse().unwrap();
  Coords { x, y }
}

fn do_case(case: &Case) -> Option<u64> {
  for acount in 0..=100 {
    let ax = case.a.x * acount;
    if ax > case.prize.x {
      break;
    }
    let diff = case.prize.x - ax;
    if diff % case.b.x == 0 {
      let bcount = diff / case.b.x;
      if case.a.y * acount + case.b.y * bcount == case.prize.y {
        return Some(acount * 3 + bcount)
      }
    }
  }
  None
}

fn main() {
  
  let mut state = A;
  let mut cases = vec![];
  let mut case = Case::new();
  
  for r in io::stdin().lines() {
    let line = r.unwrap();
    
    match state {
      A => case.a = parse_line(&line),
      B => case.b = parse_line(&line),
      Prize => case.prize = parse_line(&line),
      Blank => {
        cases.push(case);
        case = Case::new();
      }
    }
    
    state = match state {
      A => B,
      B => Prize,
      Prize => Blank,
      Blank => A
    }
  }
  
  if let Blank = state {
    cases.push(case);
  }

  let mut sum = 0;
  for c in cases {
    println!("{:?}", c);
    if let Some(s) = do_case(&c) {
      println!("Cost is {}", s);
      sum += s;
    }
  }
  println!("{}", sum);
}
