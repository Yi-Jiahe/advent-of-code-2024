#[derive(Debug, Clone)]
struct File {
  id: u32,
  size: u32,
  position: u32
}

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

fn print_files(files: &Vec<File>) {
  let mut i = 0;
  for file in files {
    for _ in 0..file.size {
      print!("{}", file.id);
    }
    if i < files.len() - 1 {
      for _ in 0..(files[i+1].position - file.position - file.size) {
        print!(".");
      }
    }
    i += 1;
  }
  print!("\n");
}

pub fn part_2(input: &str) -> String {
  // Parse input
  let mut is_file = true;
  let mut file_id = 0;
  let mut files: Vec<File> = Vec::new();
  let mut i = 0;
  for x in input.trim().chars().map(|c| c.to_digit(10).unwrap()) {
    if is_file {
      let file = File { id: file_id, size: x, position: i };
      files.push(file);
      file_id += 1;
    }

    is_file = !is_file;
    i += x;
  }

  file_id -= 1;
  while file_id > 0 {
    let mut i = files.len() - 1;
    let file;
    loop {
        if files[i].id == file_id {
          file = files[i].clone();
          break;
        }
        i -= 1;
    }
    let mut j = 0;
    for _ in 0..files.len() {
      if files[j].id == file_id {
        break;
      }
      if files[j].position >= file.position {
        break;
      }
      // There should never be the case where this loop reaches the end
      let gap = files[j+1].position - (files[j].position + files[j].size);
      if gap >= file.size {
        files[i].position = files[j].position + files[j].size;
        let file = files.remove(i);
        files.insert(j+1, file);
        break;
      }
      j += 1;
    }
    file_id -= 1;
    // print_files(&files);
  }

  let mut ans = 0;
  for i in 0..files.len() {
    let file = &files[i];
    for j in 0..file.size {
      ans += file.id as u64 * (file.position + j) as u64;
    }
  }

  ans.to_string()
}
