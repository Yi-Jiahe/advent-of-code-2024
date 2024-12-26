use std::collections::HashSet;

pub fn part_1(input: &str) -> String {
    let plots = parse_map(input);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut ans = 0;
    for y in 0..plots.len() {
        for x in 0..plots[y].len() {
          let position =  (x, y);
          if visited.contains(&position) {
            continue;
          }
            let (price, new_visited) = determine_region_price(&plots, position);
            ans += price;
            visited.extend(new_visited);
        }
    }

    ans.to_string()
}

pub fn part_2(input: &str) -> String {
    unimplemented!();
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    let mut plots: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        plots.push(line.chars().collect());
    }

    plots
}

fn determine_region_price(
    plots: &Vec<Vec<char>>,
    start: (usize, usize),
) -> (usize, HashSet<(usize, usize)>) {
    let mut stack = vec![start];
    let plant_type = plots[start.1][start.0];
    let mut visited = HashSet::new();

    let mut area = 0;
    let mut perimeter = 0;
    while !stack.is_empty() {
        let curr = stack.pop().unwrap();
        visited.insert(curr);
        area += 1;

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next_temp = (curr.0 as i32 + dx, curr.1 as i32 + dy);
            if next_temp.0 < 0
                || next_temp.0 >= plots[0].len() as i32
                || next_temp.1 < 0
                || next_temp.1 >= plots.len() as i32
            {
                perimeter += 1;
                continue;
            }

            let next = (next_temp.0 as usize, next_temp.1 as usize);
            // Only consider this node if we have not explored it before or intend to explore it
            if visited.contains(&next) || stack.contains(&next) {
                continue;
            }

            if plots[next.1][next.0] != plant_type {
                perimeter += 1;
                continue;
            }

            stack.push(next);
        }
    }

    let price = area * perimeter;
    // dbg!(area, perimeter, price, plant_type);
    (price, visited)
}
