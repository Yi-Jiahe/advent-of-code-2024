pub fn part_1(input: &str) -> String {
  let mut n_safe_levels = 0;

  for line in input.lines() {
    let report: Vec<i32> = line
      .split_whitespace()
      .map(|s| s.parse().unwrap())
      .collect();

    let safe = test_report_safety(&report);

    if safe {
      n_safe_levels += 1;
    }
  };

  n_safe_levels.to_string()
}

pub fn part_2(input: &str) -> String {
  let mut n_safe_levels = 0;

  for line in input.lines() {
    let report: Vec<i32> = line
      .split_whitespace()
      .map(|s| s.parse().unwrap())
      .collect();

    let mut safe = test_report_safety(&report);

    if safe {
      n_safe_levels += 1;
      continue;
    }

    // Problem dampener
    // It would probably be more efficient to only try removing the bad levels but the answer returns instantly anyway.
    for i in 0..report.len() {
      let mut new_report = report.clone();
      new_report.remove(i);

      if test_report_safety(&new_report) {
        safe = true;
        break;
      }
    }

    if safe {
      n_safe_levels += 1;
    }
  };

  n_safe_levels.to_string()
}

// Returns true if the report is safe, false otherwise
fn test_report_safety(report: &Vec<i32>) -> bool {
  let mut levels = report.into_iter();

  let mut prev = levels.next().unwrap();
  let mut prev_difference: Option<i32> = None;
  for level in levels {
    let difference = level - prev;

    // Check that "The levels are either all increasing or all decreasing."
    if let Some(direction) = prev_difference {
      if direction * difference < 0 {
        return false
      }
    }

    // Check that "Any two adjacent levels differ by at least one and at most three."
    if i32::abs(difference) < 1 || i32::abs(difference) > 3 {
      return false
    }

    prev = level;
    prev_difference = Some(difference);
  };

  true
}
