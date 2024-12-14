use std::io;
use std::iter::zip;
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
    one.sort();
    two.sort();
    let sum:i32 = zip(one, two).map(|(a, b)| { (a - b).abs() }).sum();
    //let mut sum = 0;
    println!("{}", sum);
}
