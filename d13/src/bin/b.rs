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

  fn prizify(&self) -> Coords {
    Coords { x: self.x + 10000000000000, y: self.y + 10000000000000 }
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
  let px = case.prize.x as f64;
  let py = case.prize.y as f64;
  let ax = case.a.x as f64;
  let ay = case.a.y as f64;
  let bx = case.b.x as f64;
  let by = case.b.y as f64;

  let foo = ay / ax;
  let bar = by / bx;
  if (foo - bar).abs() < 1e-3 {
    println!("!!! {:?}", case);
  }
  
  let noverm = (by*(px/py) - bx) / (ax - ay*(px/py));
  let m = px/(noverm*ax + bx);
  let n = m * noverm;
  // println!("n/m: {} n: {} m: {}", noverm, n, m);

  // let pxp = n*ax + m*bx;
  // let pyp = n*ay + m*by;
  // println!("pxp: {} pyp: {}", pxp, pyp);

  let ni = n.round() as u64;
  let mi = m.round() as u64;
  // println!("{} * {} + {} * {} == {}", case.a.x, ni, case.b.x, mi, case.a.x * ni + case.b.x * mi);
  // println!("{} * {} + {} * {} == {}", case.a.y, ni, case.b.y, mi, case.a.y * ni + case.b.y * mi);
  if case.prize.x == case.a.x * ni + case.b.x * mi && case.prize.y == case.a.y * ni + case.b.y * mi {
    // println!("Case good");
    Some(ni * 3 + mi)
  } else {
    // println!("Case bad");
    None
  }

  // // let prize_ratio = case.prize.x as f64 / case.prize.y as f64;
  // // let a_ratio = case.a.y as f64 / case.a.x as f64;
  // // let b_ratio = case.b.y as f64 / case.b.x as f64;
  // // println!("prize: {} a: {} b: {}", prize_ratio, a_ratio, b_ratio);
  // for acount in 0.. {
  //   let ax = case.a.x * acount;
  //   if ax > case.prize.x {
  //     break;
  //   }
  //   let diff = case.prize.x - ax;
  //   if diff % case.b.x == 0 {
  //     let bcount = diff / case.b.x;
  //     if case.a.y * acount + case.b.y * bcount == case.prize.y {
  //       return Some(acount * 3 + bcount)
  //     }
  //   }
  // }
  // None
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
      Prize => case.prize = parse_line(&line).prizify(),
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
    // println!("{:?}", c);
    if let Some(s) = do_case(&c) {
      // println!("Cost is {}", s);
      sum += s;
    }
  }
  println!("{}", sum);
}
