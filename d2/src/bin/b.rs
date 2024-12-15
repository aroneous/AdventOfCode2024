use std::io;

fn main() {
    let mut buffer = String::new();

    let mut num_safe = 0;
    while io::stdin().read_line(&mut buffer).is_ok() {
        // println!("Read line {buffer} {num_safe}");
        let record: Vec<i32> = buffer.split_whitespace()
        .map(|s| s.parse().expect("not an integer"))
        .collect();

        if record.is_empty() {
            break;
        }

        // let mut increasing = None;
        // let mut safe = true;
        for n in 0..record.len() {
            let mut last = None;
            let mut diffs: Vec<i32> = vec![];
            for (idx, num) in record.iter().enumerate() {
                if n != idx {
                    if let Some(l) = last {
                        diffs.push(num - l);
                    }
                    last = Some(num);
                }
            }
            let safe = diffs.iter().all(|&x| x >= 1 && x <= 3) || diffs.iter().all(|&x| x <= -1 && x >= -3);
            if safe {
                num_safe += 1;
                break;
            }
        }
        buffer.clear();
    }
    println!("{}", num_safe);
}