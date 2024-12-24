use std::{env, io, vec};
use regex::Regex;

#[derive(Debug)]
struct Robot {
  x: u32,
  y: u32,
  dx: i32,
  dy: i32,
}

fn parse_line(line: &str) -> Robot {
  let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
  let (_, [xstr, ystr, dxstr, dystr]) = re.captures(line).unwrap().extract();
  let x: u32 = xstr.parse().unwrap();
  let y: u32 = ystr.parse().unwrap();
  let dx: i32 = dxstr.parse().unwrap();
  let dy: i32 = dystr.parse().unwrap();
  Robot { x, y, dx, dy }
}

fn iterate(n: u32, robot: &Robot, width: u32, height: u32) -> Robot {
  let mut dx = robot.dx;
  while dx < 0 {
    dx += width as i32; 
  }
  let mut dy = robot.dy;
  while dy < 0 {
    dy += height as i32;
  }
  let newx = (robot.x as i32 + (dx * n as i32)) as u32 % width;
  let newy = (robot.y as i32 + (dy * n as i32)) as u32 % height;
  // println!("{}, {}", newx, newy);
  Robot { x: newx, y: newy, dx: robot.dx, dy: robot.dy }
}

fn make_grid(robots: &Vec<Robot>, width: u32, height: u32) -> Vec<Vec<i32>> {
  let mut g = vec![];
  for _ in 0..height {
    let row = vec![0; width as usize];
    g.push(row);
  }
  for r in robots {
    g[r.y as usize][r.x as usize] += 1;
  }
  g
}
fn print_grid(g: &Vec<Vec<i32>>) {
  for row in g {
    for cell in row {
      if *cell == 0 {
        print!(".");
      } else {
        print!("{}", cell);
      }
    }
    println!();
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 3 {
    panic!("Invoke with <width> and <height>");
  }
  let width = &args[1].parse::<u32>().unwrap();
  let height = &args[2].parse::<u32>().unwrap();
  // println!("{} x {}", *width, *height);

  let mut robots = vec![];
  for r in io::stdin().lines() {
    let line = r.unwrap();
    
    let robot = parse_line(&line);
    robots.push(robot);
  }

  // println!("{:?}", robots);

  // print_grid(&robots, *width, *height);
  // println!();

  // let midw = width / 2;
  // let midh = height / 2;

  let step = 1;

  // let symthresh = ((width * height) as f32 * 0.9) as i32;

  let mut next = robots;
  for iter in (1..=200000).step_by(step) {
    let mut nexter = vec![];
    for robot in next {
      nexter.push(iterate(step as u32, &robot, *width, *height));
    }
    let g = make_grid(&nexter, *width, *height);

    let mut found = false;
    'row: for row in g.iter().rev() {
      let mut cnt = 0;
      for cell in row {
        if *cell > 0 {
          cnt += 1;
          if cnt >= 10 {
            found = true;
            break 'row;
          }
        } else {
          cnt = 0;
        }
      }
    }
    if found {
      println!("Iteration {}:", iter);
      print_grid(&g);
    }

    // let symcount = g.iter().fold(0, |acc, row| {
    //   acc + row[0..midw as usize].iter().enumerate().fold(0, |acc2, (idx, cell)| {
    //     acc2 + if *cell == row[row.len() - 1 - idx] { 1 } else { 0 }
    //   })
    // });
    // if symcount >= symthresh {
    //   println!("Iteration {}:", iter);
    //   print_grid(&g);
    // }
    // if g.iter().all(|row| {
    //   row[0..midw as usize].iter().enumerate().all(|(idx, cell)| {
    //     *cell == row[row.len() - 1 - idx]
    //   })
    // }) {
    //   println!("Iteration {}:", iter);
    //   print_grid(&g);
    // }

    // println!("Iteration {}:", iter);
    // print_grid(&g);

    // let left_ok = g[0][0..midw as usize].iter().all(|cell| *cell == 0);
    // let right_ok = g[0][(midw as usize + 1)..(g[0].len())].iter().all(|cell| *cell == 0);
    // if g[0][midw as usize] == 1 && left_ok && right_ok {
    //   println!("Iteration {}:", iter);
    //   print_grid(&nexter, *width, *height);
    //   // break;
    // }
    next = nexter;
  }

  // let mut quads: Vec<u32> = vec![0; 4];
  // let midw = width / 2;
  // let midh = height / 2;
  // for robot in end {
  //   let wq;
  //   if robot.x < midw {
  //     wq = 0;
  //   } else if robot.x > midw {
  //     wq = 1;
  //   } else {
  //     continue;
  //   }
  //   let hq;
  //   if robot.y < midh {
  //     hq = 0;
  //   } else if robot.y > midh {
  //     hq = 1;
  //   } else {
  //     continue;
  //   }
  //   quads[wq * 2 + hq] += 1;
  // }

  // let sum = quads[0] * quads[1] * quads[2] * quads[3];

  // println!("{}", sum);
}
