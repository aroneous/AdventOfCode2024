use std::io;

fn test(value: u128, current: u128, rest: &[u128]) -> bool {
  if current > value {
    // print!("Too big: {} > {}", current, value);
    return false;
  }
  if rest.is_empty() {
    // println!("is_empty: current == {}", current);
    return current == value;
  } else {
    let next = rest[0];
    let mut mult = 1;
    let mut x = next;
    while x > 0 {
      mult *= 10;
      x /= 10;
    }
    let concat = current * mult + next;
  // println!("Testing next == {}: {} and {}", next, current + next, current * next);
    return test(value, concat, &rest[1..])
        || test(value, current + next, &rest[1..])
        || test(value, current * next, &rest[1..]);
  }
}
fn main() {
  let mut sum = 0;
  let mut buffer = String::new();
  while io::stdin().read_line(&mut buffer).is_ok() {
    if buffer.trim().is_empty() {
      break;
    }

    let case: Vec<u128> = buffer.trim().split_whitespace()
    .map(|s| s.trim_end_matches(":"))
    .map(|s| s.parse::<u128>().unwrap())
    .collect();

    let value = case[0];

    // println!("Doing case {}", buffer.trim());
    if test(value, 0, &case[1..]) {
      // println!("Case {} is good", buffer.trim());
      sum += value;
    }

    buffer.clear();
  }

  println!("{}", sum);
}
