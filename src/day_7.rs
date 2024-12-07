pub fn part_1(input: &str) -> String {
    let mut total_calibration_result = 0;

    for line in input.lines() {
        let values = line.split_once(':').unwrap();
        let test_value = values.0.trim().parse::<i64>().unwrap();
        let numbers = values
            .1
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        if place_operator(test_value, numbers[0], 0, &numbers, 1) {
            total_calibration_result += test_value;
        }
    }

    total_calibration_result.to_string()
}

pub fn part_2(input: &str) -> String {
  let mut total_calibration_result = 0;

  for line in input.lines() {
      let values = line.split_once(':').unwrap();
      let test_value = values.0.trim().parse::<i64>().unwrap();
      let numbers = values
          .1
          .trim()
          .split_whitespace()
          .map(|s| s.parse::<i64>().unwrap())
          .collect::<Vec<i64>>();

      if place_operator(test_value, numbers[0], 0, &numbers, 2) {
          total_calibration_result += test_value;
      }
  }

  total_calibration_result.to_string()
}

fn place_operator(test_value: i64, current_value: i64, i: usize, numbers: &Vec<i64>, part: i32) -> bool {
    if i == numbers.len() - 1 {
        return current_value == test_value;
    }

    let part_1_check = place_operator(test_value, current_value + numbers[i + 1], i + 1, numbers, part)
        || place_operator(test_value, current_value * numbers[i + 1], i + 1, numbers, part);
      
    if part == 1 {
        return part_1_check;
    }

    return part_1_check
        || place_operator(test_value, format!("{}{}", current_value.to_string(), numbers[i + 1].to_string()).parse::<i64>().unwrap(), i + 1, numbers, part);
}
