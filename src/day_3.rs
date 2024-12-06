use regex::Regex;

pub fn part_1(input: &str) -> String {
  let mut result = 0;

  let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

  for line in input.lines() {
    for cap in re.captures_iter(line) {
      let x = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
      let y = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();

      result += x * y;
    }
  }

  result.to_string()
}

pub fn part_2(input: &str) -> String { todo!()}
