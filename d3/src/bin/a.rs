use std::io;
use regex::Regex;

fn main() {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut buffer = String::new();
    let mut sum = 0;
    while io::stdin().read_line(&mut buffer).is_ok() {
      if buffer.is_empty() {
        break;
      }
      for (_, [astr, bstr]) in re.captures_iter(&buffer).map(|c| c.extract()) {
        let a = astr.parse::<i32>().unwrap();
        let b = bstr.parse::<i32>().unwrap();
        let prod = a * b;
        sum += prod;
      }
      buffer.clear();
    }
    println!("{}", sum);
}
