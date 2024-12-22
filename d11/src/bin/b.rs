use std::{collections::HashMap, io};

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

fn count_for_num(val: u128, iter_remain: i32, cache: &mut HashMap<(u128, i32), u128>) -> u128 {
  if iter_remain <= 0 {
    1
  } else {
    if let Some(cached) = cache.get(&(val, iter_remain)) {
      *cached
    } else {
      let count;
      if val == 0 {
        count = count_for_num(1, iter_remain - 1, cache);
      } else if let Some((first, second)) = try_split(val) {
        count = count_for_num(first, iter_remain - 1, cache)
        + count_for_num(second, iter_remain - 1, cache)
      } else {
        count = count_for_num(val * 2024, iter_remain - 1, cache);
      }
      cache.insert((val, iter_remain), count);
      count
    }
  }
}

fn main() {
  let mut buffer = String::new();
  
  while io::stdin().read_line(&mut buffer).is_ok() {
    // println!("Read line {buffer} {num_safe}");
    let record: Vec<u128> = buffer
    .trim()
    .split_whitespace()
    .map(|s| s.parse().expect("not an integer"))
    .collect();
    
    if record.is_empty() {
      break;
    }

    let mut cache = HashMap::new();

    let mut count = 0;
    for val in record {
      count += count_for_num(val, 75, &mut cache);
    }

    println!("{}", count);

    buffer.clear();
  }
}
