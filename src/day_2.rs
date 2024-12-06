pub fn part_1(input: &str) -> String {
  let mut n_safe_levels = 0;

  for line in input.lines() {
    let mut values = line.split_whitespace().into_iter();

    let mut safe = true;

    let mut prev = values.next().unwrap().parse::<i32>().unwrap();
    let mut prev_difference: Option<i32> = None;
    for value in values {
      let value = value.parse::<i32>().unwrap();

      let difference = value - prev;

      // Check that "The levels are either all increasing or all decreasing."
      if let Some(direction) = prev_difference {
        if direction * difference < 0 {
          safe = false;
          break;
        }
      }

      // Check that "Any two adjacent levels differ by at least one and at most three."
      if i32::abs(difference) < 1 || i32::abs(difference) > 3 {
        safe = false;
        break;
      }

      prev = value;
      prev_difference = Some(difference);
    };

    if safe {
      n_safe_levels += 1;
    }
  };

  n_safe_levels.to_string()
}

pub fn part_2(input: &str) -> String { todo!() }
