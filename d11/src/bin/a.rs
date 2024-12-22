use std::io;

fn try_split(num: u128) -> Option<(u128, u128)> {
  let mut digits = vec![];
  let mut n = num;
  while n > 0 {
    digits.push(n % 10);
    n /= 10;
  }

  if digits.len() % 2 == 0 {
    let mut second = 0;
    let half = digits.len() / 2;
    for _ in 0..half {
      second *= 10;
      second += digits.pop().unwrap();
    }
    let mut first = 0;
    loop {
      match digits.pop() {
        Some(d) => {
          first *= 10;
          first += d;
        },
        None => break
      }
    }
    Some((first, second))
  } else {
    None
  }
}

fn main() {
  let mut buffer = String::new();
  
  while io::stdin().read_line(&mut buffer).is_ok() {
    // println!("Read line {buffer} {num_safe}");
    let mut record: Vec<u128> = buffer
    .trim()
    .split_whitespace()
    .map(|s| s.parse().expect("not an integer"))
    .collect();
    
    if record.is_empty() {
      break;
    }

    for _iter in 1..=25 {
      let mut next_record = vec![];
      for val in record {
        if val == 0 {
          next_record.push(1);
        } else if let Some((first, second)) = try_split(val) {
          next_record.push(first);
          next_record.push(second);
        } else {
          next_record.push(val * 2024);
        }
      }
      // println!("{}: {:?}", _iter, next_record);
      record = next_record;
    }

    println!("{}", record.len());

    buffer.clear();
  }
}
