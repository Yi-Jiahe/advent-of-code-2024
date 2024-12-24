pub fn part_1(input: &str) -> String {

  // Parse input
  let mut is_file = true;
  let mut blocks: Vec<Option<u32>> = Vec::new();
  let mut file_id = 0;
  for x in input.trim().chars().map(|c| c.to_digit(10).unwrap()) {
    if is_file {
      for _ in 0..x {
        blocks.push(Some(file_id));
      }
      file_id += 1;
      is_file = false; 
      continue;
    }

    for _ in 0..x {
      blocks.push(None);
    }
    is_file = true;
  } 

  // Shift files to front
  let mut i = 0;
  let mut j = blocks.len() - 1;
  'outer: loop {
    if let Some(x) = blocks[j] {
      // Search for next empty block
      while blocks[i] != None {
        i += 1;
        if i == j {
          break 'outer;
        }
      }
      blocks[i] = Some(x);
      blocks[j] = None;
      i += 1;
    }
    j -= 1;

    if i == j {
      break;
    }
  }

  let mut ans = 0;
  for i in 0..blocks.len() {
    if let Some(x) = blocks[i] {
      ans += i * x as usize;
    }
  }
  
  ans.to_string()
}

pub fn part_2(input: &str) -> String {
  unimplemented!();   
}
