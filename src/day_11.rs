pub fn part_1(input: &str) -> String {
  let mut stones = input.split(' ').map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();

  let mut n_blinks = 0;
  while n_blinks < 25 {
    let mut i = 0;   
    while i < stones.len() {
      let stone = stones[i];
    
      if stone == 0 {
        stones[i] = 1;
      
        i += 1;
      } else if stone.to_string().len() % 2 == 0  {
        let stone_str = stone.to_string();
        let len = stone_str.len();
        let left = &stone_str[0..len/2].parse::<u64>().unwrap();
        let right = &stone_str[len/2..len].parse::<u64>().unwrap();

        stones.remove(i);
        stones.insert(i, *right);
        stones.insert(i, *left);
    
        i += 2;
      } else {
        stones[i] = stone * 2024;

        i += 1;
      }
    }

    n_blinks += 1;
  }

  stones.len().to_string()
}

pub fn part_2(input: &str) -> String {
  unimplemented!()
}
