use std::io;

fn test(value: u128, current: u128, rest: &[u128]) -> bool {
  if current > value {
    return false;
  }
  if rest.is_empty() {
    return current == value;
  } else {
    let next = rest[0];
    return test(value, current + next, &rest[1..]) || test(value, current * next, &rest[1..]);
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

    if test(value, 0, &case[1..]) {
      sum += value;
    }

    buffer.clear();
  }

  println!("{}", sum);
}
