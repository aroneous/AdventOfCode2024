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

fn print_grid(robots: &Vec<Robot>, width: u32, height: u32) {
  let mut g = vec![];
  for _ in 0..height {
    let row = vec![0; width as usize];
    g.push(row);
  }
  for r in robots {
    g[r.y as usize][r.x as usize] += 1;
  }
  for row in g {
    for cell in row {
      if cell == 0 {
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

  print_grid(&robots, *width, *height);
  println!();

  let mut end = vec![];
  for robot in robots {
    end.push(iterate(100, &robot, *width, *height));
  }

  print_grid(&end, *width, *height);

  let mut quads: Vec<u32> = vec![0; 4];
  let midw = width / 2;
  let midh = height / 2;
  for robot in end {
    let wq;
    if robot.x < midw {
      wq = 0;
    } else if robot.x > midw {
      wq = 1;
    } else {
      continue;
    }
    let hq;
    if robot.y < midh {
      hq = 0;
    } else if robot.y > midh {
      hq = 1;
    } else {
      continue;
    }
    quads[wq * 2 + hq] += 1;
  }

  let sum = quads[0] * quads[1] * quads[2] * quads[3];

  println!("{}", sum);
}
