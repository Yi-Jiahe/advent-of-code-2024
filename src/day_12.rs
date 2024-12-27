use std::collections::{HashMap, HashSet};

pub fn part_1(input: &str) -> String {
    let plots = parse_map(input);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut ans = 0;
    for y in 0..plots.len() {
        for x in 0..plots[y].len() {
            let position = (x, y);
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

fn determine_discounted_region_price(
    plots: &Vec<Vec<char>>,
    start: (usize, usize),
) -> (usize, HashSet<(usize, usize)>) {
    let mut stack = vec![start];
    let plant_type = plots[start.1][start.0];
    let mut visited = HashSet::new();

    let mut area = 0;
    // Map of sides grouped by which way it is facing, in to out
    // A side is a vector of adjacent fences
    let mut sides: HashMap<(i32, i32), Vec<Vec<(usize, usize)>>> = HashMap::from([
        ((-1, 0), Vec::new()),
        ((1, 0), Vec::new()),
        ((0, -1), Vec::new()),
        ((0, 1), Vec::new()),
    ]);
    while !stack.is_empty() {
        let curr = stack.pop().unwrap();
        visited.insert(curr);
        area += 1;

        for direction in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (dx, dy) = direction;
            let next_temp = (curr.0 as i32 + dx, curr.1 as i32 + dy);
            if next_temp.0 < 0
                || next_temp.0 >= plots[0].len() as i32
                || next_temp.1 < 0
                || next_temp.1 >= plots.len() as i32
            {
                add_fence(&mut sides, curr, direction);
                continue;
            }

            let next = (next_temp.0 as usize, next_temp.1 as usize);
            // Only consider this node if we have not explored it before or intend to explore it
            if visited.contains(&next) || stack.contains(&next) {
                continue;
            }

            if plots[next.1][next.0] != plant_type {
                add_fence(&mut sides, curr, direction);
                continue;
            }

            stack.push(next);
        }
    }

    let price = area * sides.len();
    // dbg!(area, perimeter, price, plant_type);
    (price, visited)
}

fn add_fence(
    sides: &mut HashMap<(i32, i32), Vec<Vec<(usize, usize)>>>,
    position: (usize, usize),
    direction: (i32, i32),
) {
    let mut sections = sides.get(&direction).unwrap().clone();
    for (i, section) in sides.get(&direction).unwrap().iter().enumerate() {
        for fence in section {
            // WIP: The plan is to identify how many sections are adjacent
            // if none, create new section
            // if one, add to section
            // if two, join the two sections and add to the new section
            // the last one is tricky but what I'm thinking is to keep the sections ordered and the fences in each section ordered such that its joining intervals.
            // I'm also thinking of splitting the map by direction and row/column, so inside each key is intervals on a line.

            // Fences facing left or right
            if direction == (-1, 0) || direction == (1, 0) {
                // x must be the same for the fence to be part of the same side
                if position.0 != fence.0 {
                    break;
                }
                // Only add the fence to the side if there is a fence directly adjacent to it
                if (position.1 > fence.1 && position.1 - fence.1 == 1)
                    || (position.1 < fence.1 && fence.1 - position.1 == 1)
                {
                    sections[i].push(position);
                    sides.insert(direction, sections);
                    return;
                }
            }
            if direction == (0, -1) || direction == (0, 1) {
                if position.1 != fence.1 {
                    break;
                }
                // Only add the fence to the side if there is a fence directly adjacent to it
                if (position.0 > fence.0 && position.0 - fence.0 == 1)
                    || (position.0 < fence.0 && fence.0 - position.0 == 1)
                {
                    sections[i].push(position);
                    sides.insert(direction, sections);
                    return;
                }
            }
        }
    }
}
