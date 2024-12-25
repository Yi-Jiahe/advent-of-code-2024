use std::collections::HashSet;

pub fn part_1(input: &str) -> String {
    let topographic_map = parse_map(input);

    let mut ans = 0;
    for y in 0..topographic_map.len() {
        for x in 0..topographic_map[y].len() {
            if topographic_map[y][x] == 0 {
                ans += score_trailhead(&topographic_map, (x, y));
            }
        }
    }

    ans.to_string()
}

pub fn part_2(input: &str) -> String {
    let topographic_map = parse_map(input);

    let mut ans = 0;
    for y in 0..topographic_map.len() {
        for x in 0..topographic_map[y].len() {
            if topographic_map[y][x] == 0 {
                ans += rate_trailhead(&topographic_map, (x, y));
            }
        }
    }

    ans.to_string()
}

fn parse_map(input: &str) -> Vec<Vec<u32>> {
    let mut topographic_map: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        topographic_map.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    topographic_map
}

fn score_trailhead(topographic_map: &Vec<Vec<u32>>, position: (usize, usize)) -> usize {
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut peaks: HashSet<(usize, usize)> = HashSet::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    stack.push(position);

    while !stack.is_empty() {
        let curr = stack.pop().unwrap();
        visited.insert(curr);
        let curr_height = topographic_map[curr.1][curr.0];

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next_temp = (curr.0 as i32 + dx, curr.1 as i32 + dy);
            if next_temp.0 < 0
                || next_temp.0 >= topographic_map[0].len() as i32
                || next_temp.1 < 0
                || next_temp.1 >= topographic_map.len() as i32
            {
                continue;
            }

            let next = (next_temp.0 as usize, next_temp.1 as usize);
            // Only consider this node if we have not explored it before or intend to explore it
            if visited.contains(&next) || stack.contains(&next) {
                continue;
            }

            let next_height = topographic_map[next.1 as usize][next.0 as usize];
            if next_height > curr_height && next_height - curr_height == 1 {
                if next_height == 9 {
                    peaks.insert(next);
                    continue;
                }

                stack.push(next);
            }
        }
    }

    peaks.len()
}

fn rate_trailhead(topographic_map: &Vec<Vec<u32>>, position: (usize, usize)) -> u32 {
  let mut rating = 0;

  let curr_height = topographic_map[position.1][position.0];

  for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
    let next_temp = (position.0 as i32 + dx, position.1 as i32 + dy);
    if next_temp.0 < 0
      || next_temp.0 >= topographic_map[0].len() as i32
      || next_temp.1 < 0
      || next_temp.1 >= topographic_map.len() as i32
    {
      continue;
    }

    let next = (next_temp.0 as usize, next_temp.1 as usize);
    let next_height = topographic_map[next.1 as usize][next.0 as usize];
    if next_height > curr_height && next_height - curr_height == 1 {
      if next_height == 9 {
        rating += 1;
        continue;
      }
      // No need to keep track of past because the trails being strictly increasing will take care of that
      rating += rate_trailhead(topographic_map, next);
    }
  }

  rating
}