use std::{collections::HashMap, io};

fn main() {
  let mut not_before: HashMap<u32, Vec<u32>> = HashMap::new();
  let mut buffer = String::new();
  while io::stdin().read_line(&mut buffer).is_ok() {
    if buffer.trim().is_empty() {
      break;
    }
    // println!("{}", buffer);
    let parts: Vec<u32> = buffer.split('|').map(|s| s.trim().parse().unwrap()).collect();
    let before = parts[0];
    let after = parts[1];
    // let b = before.parse::<u32>().unwrap();
    // let a = after.parse::<u32>().unwrap();
    not_before.entry(before).and_modify(|v| v.push(after)).or_insert(vec![after]);
    // not_before.insert(before.parse::<u32>().unwrap(), after.parse::<u32>().unwrap());
    buffer.clear();
  }

  // println!("{:?}", not_before);
  buffer.clear();
  let mut sum = 0;
  while io::stdin().read_line(&mut buffer).is_ok() {
    if buffer.is_empty() {
      break;
    }
    let order: Vec<u32> = buffer.split(',').map(|a| a.trim().parse().unwrap()).collect();

    // println!("Order: {:?}", order);
    let mut ok = true;
    for (idx, item) in order[1..].iter().enumerate() {
      if let Some(musts) = not_before.get(item) {
        // println!("{}: Musts for {}: {:?}", idx, item, musts);
        if order[0..=idx].iter().find(|o| musts.contains(*o)).is_some() {
          ok = false;
          break;
        }
      }
    }
    if ok {
      // println!("ok");
      // println!("{}", buffer);
      let mid = order[order.len() / 2];
      sum += mid;
    }

    buffer.clear();
  }

  println!("{}", sum);
}
