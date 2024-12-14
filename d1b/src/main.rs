use std::io;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();

    let mut buffer = String::new();

    let mut one = vec![];
    let mut two = vec![];
    while io::stdin().read_line(&mut buffer).is_ok() {
        if !re.is_match(&buffer) {
            break;
        }
        let cap = re.captures(&buffer).unwrap();
        let a = cap[1].parse::<i32>().unwrap();
        let b = cap[2].parse::<i32>().unwrap();
        one.push(a);
        two.push(b);
        buffer.clear();
    }
    let mut counts = HashMap::new();
    for ele in two {
        counts.entry(ele).and_modify(|count| *count += 1).or_insert(1);
    }
    let sum:i32 = one.iter().map(|num| num * counts.get(num).unwrap_or(&0)).sum();
    println!("{}", sum);
}
