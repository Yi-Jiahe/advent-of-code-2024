use std::collections::{HashMap, HashSet};

pub fn part_1(input: &str) -> String {
    let (mut m, mut n) = (0, 0);
    let mut obstructions: HashSet<(usize, usize)> = HashSet::new();
    let mut position: (usize, usize) = (0, 0);
    let mut direction: (i32, i32) = (0, 1);
    let mut distinct_positions: HashSet<(usize, usize)> = HashSet::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if i == 0 {
                n = line.trim().len();
            }

            match c {
                '#' => {
                    obstructions.insert((j, i));
                }
                '^' => {
                    position = (j, i);
                    direction = (0, -1);
                }
                'v' => {
                    position = (j, i);
                    direction = (0, 1);
                }
                '>' => {
                    position = (j, i);
                    direction = (1, 0);
                }
                '<' => {
                    position = (j, i);
                    direction = (-1, 0);
                }
                _ => {}
            }
        }
        m = i + 1;
    }

    // Count the initial position
    distinct_positions.insert(position);

    let right_turn = HashMap::from([
      ((0, -1), (1, 0)),
      ((1, 0), (0, 1)), 
      ((0, 1), (-1, 0)), 
      ((-1, 0), (0, -1))
    ]);
    loop {
      let next_position_i32 = (position.0 as i32 + direction.0, position.1 as i32 + direction.1);

      // Guard moves out of the map
      if next_position_i32.0 >= n as i32 || next_position_i32.0 < 0 || next_position_i32.1 >= m as i32 || next_position_i32.1 < 0 {
        break;
      }

      let next_position = (next_position_i32.0 as usize, next_position_i32.1 as usize);

      if obstructions.contains(&next_position) {
        direction = *right_turn.get(&direction).unwrap();
        continue;
      }
      
      distinct_positions.insert(next_position);
      position = next_position;
    }

    // There's a bug causing an off by one error
    distinct_positions.len().to_string()
}

pub fn part_2(input: &str) -> String {
    todo!()
}
