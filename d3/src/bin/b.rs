use std::io;
use regex::Regex;

fn main() {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\(\d+,\d+\)").unwrap();
    let re2 = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut buffer = String::new();
    let mut sum = 0;
    let mut enabled = true;
    while io::stdin().read_line(&mut buffer).is_ok() {
      if buffer.is_empty() {
        break;
      }
      for m in re.find_iter(&buffer) {
        let c = m.as_str();
        if c == "do()" {
          enabled = true;
        } else if c == "don't()" {
          enabled = false;
        } else if enabled {
          let caps = re2.captures(c).unwrap();
          let a = &caps[1].parse::<i32>().unwrap();
          let b = &caps[2].parse::<i32>().unwrap();
          let prod = a * b;
          sum += prod;
        }
      }
      buffer.clear();
    }
    println!("{}", sum);
}
