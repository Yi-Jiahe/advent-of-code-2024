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

pub fn part_2(input: &str) -> String {
    let mut result = 0;

    let re = Regex::new(r"(mul)\((\d+),(\d+)\)|(do)\(\)|(don't)\(\)").unwrap();

    let mut enabled = true;

    for line in input.lines() {
        for cap in re.captures_iter(line) {
            if let Some(_) = cap.get(4) {
              enabled = true;
            }
            if let Some(_) = cap.get(5) {
              enabled = false;   
            }
            if let Some(_) = cap.get(1) {
              if !enabled {
                  continue;
              }
              let x = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
              let y = cap.get(3).unwrap().as_str().parse::<i32>().unwrap();

              result += x * y;
            }
        }
    }

    result.to_string()
}
