use std::io;

fn print_disc(disc: &Vec<Option<i32>>) {
  for block in disc {
    let c = match block {
      None => ".".to_string(),
      Some(val) => val.to_string()
    };
    print!("{}", c);
  }
  println!();
}

fn clear_end(disc: &mut Vec<Option<i32>>) {
  while !disc.is_empty() && disc[disc.len() - 1] == None {
    disc.pop();
  }
}

fn main() {
  let mut buffer = String::new();
  while io::stdin().read_line(&mut buffer).is_ok() {
    if buffer.trim().is_empty() {
      break;
    }

    let mut record = vec![];
    for c in buffer.trim().bytes() {
      let val = c - 48;
      record.push(val);
    }

    // println!("{:?}", record);

    let mut is_file = true;
    let mut file_id = 0;
    let mut disc = vec![];
    for val in record {
      let block;
      if is_file {
        block = Some(file_id);
        file_id += 1;
      } else {
        block = None;
      }
      for _ in 0..val {
        disc.push(block);
      }
      is_file = !is_file;
    }

    // print_disc(&disc);

    clear_end(&mut disc);
    let mut ptr = 0;
    while ptr < disc.len() - 1 {
      if disc[ptr] == None {
        disc[ptr] = disc[disc.len() - 1];
        disc.pop();
        clear_end(&mut disc);
      }
      ptr += 1;
    }

    // print_disc(&disc);

    let sum = disc.iter().enumerate().fold(0, |acc, (idx, val)|  acc + (idx * val.unwrap() as usize));
    println!("{}", sum);

    buffer.clear();
  }

  // println!("{:?}", a);
}
