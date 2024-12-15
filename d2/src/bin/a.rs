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

        let mut last = None;
        let mut increasing = None;
        let mut safe = true;
        for num in record {
            safe = match last {
                Some(prev) => {
                    increasing = increasing.or(Some(num > prev));
                    let inc = increasing.unwrap();
                    let diff = if inc { num - prev } else { prev - num };
                    diff >= 1 && diff <= 3
                },
                None => true
            };
            last = Some(num);
            if !safe {
                break;
            }
        }
        if safe {
            num_safe += 1;
        }
        buffer.clear();
    }
    println!("{}", num_safe);
}