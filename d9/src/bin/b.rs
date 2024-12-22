use std::io;

#[derive(Clone, Copy, Debug)]
enum Section {
  Free(usize),
  File {size: usize, id: u32 },
}

use Section::*;

fn print_disc(disc: &Vec<Section>) {
  for block in disc {
    let c = match block {
      Free(size) => ".".repeat(*size),
      File { size, id } => id.to_string().repeat(*size)
    };
    print!("{}", c);
  }
  println!();
}

fn replace_with_free(disc: &mut Vec<Section>, start: usize, end: usize, size: usize) {
  disc[start] = Free(size);
  for idx in (start + 1)..end {
    disc.remove(idx);
  }
}

fn collapse_free(disc: &mut Vec<Section>) {
  let mut done = true;
  while !done {
    let mut start_free = None;
    let mut sum = 0;
    for (idx, s) in disc.iter().enumerate() {
      match s {
        Free(size) => {
          if start_free.is_none() {
            start_free = Some(idx);
          }
          sum += size;
        },
        File { size: _, id: _ }  => {
          if let Some(start) = start_free {
            replace_with_free(disc, start, idx, sum);
            done = false;
            start_free = None;
            break;
          }
        }
      }
    }
    if let Some(start) = start_free {
      replace_with_free(disc, start, disc.len(), sum);
    }
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
        block = File { id: file_id, size: val.into() };
        file_id += 1;
      } else {
        block = Free(val.into());
      }
      disc.push(block);
      is_file = !is_file;
    }

    // print_disc(&disc);

    let max_file_id = file_id - 1;
    // for section in disc.iter.rev() {
    //   if let File { id: file_id, size: _ } = section {
    //     curr_file_id = file_id;
    //     break;
    //   }
    // }

    // print_disc(&disc);

    for curr_file_id in (0..=max_file_id).rev() {
      // println!("Processing file {}", curr_file_id);
      let mut file_idx_opt = None;
      let mut size = 0;
      if let Some((idx, File { id: _, size: fsize })) =
      disc.iter().enumerate().find(|(_, s)| {
        // println!("Evaluating {:?}", s);
        match s {
          File { id: i, size: _} => *i == curr_file_id,
          Free(_) => false
        }
      }) {
        file_idx_opt = Some(idx);
        size = *fsize;
      }

      if let Some(file_idx) = file_idx_opt {
        // println!("Found file at idx {}", file_idx);
        let mut free_idx_opt = None;
        let mut free_size = 0;
        if let Some((free_idx, Free(_free_size))) =
        disc.iter().enumerate().find(|(_, s)| {
          match s {
            Free(free_size) => *free_size >= size,
            _ => false
          }
        }) {
          free_idx_opt = Some(free_idx);
          free_size = *_free_size;
        }
        if let Some(free_idx) = free_idx_opt {
          if free_idx < file_idx {
            let file = disc[file_idx];
            // println!("Move {:?} from {} to {}", file, file_idx, free_idx);
            disc[free_idx] = file;
            disc[file_idx] = Free(size);
            let remaining_free = free_size - size;
            if remaining_free > 0 {
              disc.insert(free_idx + 1, Free(remaining_free));
            }
            collapse_free(&mut disc);
          }
        }
      }
    }

    // print_disc(&disc);

    let mut checksum = 0;
    let mut start_block = 0;
    for s in &disc {
      match s {
        Free(size) => start_block += size,
        File { size, id } => {
          for idx in 0..*size {
            checksum += (start_block + idx) * *id as usize;
          }
          start_block += size;
        }
      }
    }
  
    // let sum = disc.iter().enumerate().fold(0, |acc, (idx, val)|  acc + (idx * val.unwrap() as usize));
    println!("{}", checksum);

    buffer.clear();
  }

  // println!("{:?}", a);
}
