use std::{collections::{HashMap, HashSet}, io};

fn main() {
  let mut a: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

  let mut buffer = String::new();
  let mut y = 0;
  let mut height = 0;
  let mut width: i32 = 0;
  while io::stdin().read_line(&mut buffer).is_ok() {
    if buffer.trim().is_empty() {
      break;
    }

    height += 1;
    width = buffer.trim().len() as i32;

    for (x, cell) in buffer.trim().chars().enumerate() {
      if cell != '.' {
        a.entry(cell).and_modify(|e| e.push((x as i32, y as i32)))
        .or_insert(vec![(x as i32, y as i32)]);
      }
    }

    buffer.clear();
    y += 1;
  }

  // println!("{:?}", a);

  let mut ans = HashSet::new();
  for ants in a.values() {
    for (ione, (x1, y1)) in ants.iter().enumerate() {
      for (x2, y2) in ants[(ione + 1)..].iter() {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let mut anx1 = *x1;
        let mut any1 = *y1;
        while anx1 >= 0 && anx1 < width && any1 >= 0 && any1 < height {
          ans.insert((anx1, any1));
          anx1 -= dx;
          any1 -= dy;
        }

        let mut anx2 = *x2;
        let mut any2 = *y2;
        while anx2 >= 0 && anx2 < width && any2 >= 0 && any2 < height {
          ans.insert((anx2, any2));
          anx2 += dx;
          any2 += dy;
        }
      }
    }
  }

  println!("{}", ans.len());
}
