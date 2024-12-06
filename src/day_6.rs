use std::collections::HashSet;

pub fn part_1(input: &str) -> String {
    let ((m, n), obstructions, mut position, mut direction) = parse_map(input);
    let mut distinct_positions: HashSet<(usize, usize)> = HashSet::new();

    loop {
        distinct_positions.insert(position);

        let next_position_i32 = (
            position.0 as i32 + direction.0,
            position.1 as i32 + direction.1,
        );

        // Guard moves out of the map
        if next_position_i32.0 >= n as i32
            || next_position_i32.0 < 0
            || next_position_i32.1 >= m as i32
            || next_position_i32.1 < 0
        {
            break;
        }

        let next_position = (next_position_i32.0 as usize, next_position_i32.1 as usize);

        if obstructions.contains(&next_position) {
            direction = turn_right(direction);
            continue;
        }

        position = next_position;
    }

    distinct_positions.len().to_string()
}

pub fn part_2(input: &str) -> String {
    let ((m, n), obstructions, mut position, mut direction) = parse_map(input);
    let mut distinct_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut distinct_states: HashSet<((usize, usize), (i32, i32))> = HashSet::new();

    let mut possible_obstruction_positions: HashSet<(usize, usize)> = HashSet::new();

    loop {
      distinct_positions.insert(position);
        distinct_states.insert((position, direction));

        let next_position_i32 = (
            position.0 as i32 + direction.0,
            position.1 as i32 + direction.1,
        );

        // Guard moves out of the map
        if next_position_i32.0 >= n as i32
            || next_position_i32.0 < 0
            || next_position_i32.1 >= m as i32
            || next_position_i32.1 < 0
        {
            break;
        }

        let next_position = (next_position_i32.0 as usize, next_position_i32.1 as usize);

        if obstructions.contains(&next_position) {
            direction = turn_right(direction);
            distinct_states.insert((position, direction));
            continue;
        }

        let new_obstruction_position = next_position.clone();
        // The new obstruction can't be a spot that has been walked past
        // Apparently the historians can't place an obstruction once the guard has started moving
        // Although logically, if they can place one right in front of the guard...
        if !distinct_positions.contains(&new_obstruction_position) {
            // Place a new obstruction in front of the path to test if its a feasible location
        let mut new_obstructions = obstructions.clone();
        new_obstructions.insert(new_obstruction_position);
        let mut alternate_distinct_states = distinct_states.clone();
        // The current state has to be distinct in order for the original path not to be a loop
        // Remove it in order to not break the search immediately
        alternate_distinct_states.remove(&(position, direction));
        let mut alternate_position = position;
        let mut alternate_direction = direction;
        loop {
            if alternate_distinct_states.contains(&(alternate_position, alternate_direction)) {
                possible_obstruction_positions.insert(new_obstruction_position);
                break;
            }

            alternate_distinct_states.insert((alternate_position, alternate_direction));

            let alternate_next_position_i32 = (
                alternate_position.0 as i32 + alternate_direction.0,
                alternate_position.1 as i32 + alternate_direction.1,
            );

            // Guard moves out of the map
            if alternate_next_position_i32.0 >= n as i32
                || alternate_next_position_i32.0 < 0
                || alternate_next_position_i32.1 >= m as i32
                || alternate_next_position_i32.1 < 0
            {
                break;
            }

            let alternate_next_position = (
                alternate_next_position_i32.0 as usize,
                alternate_next_position_i32.1 as usize,
            );

            if new_obstructions.contains(&alternate_next_position) {
                alternate_direction = turn_right(alternate_direction);
                continue;
            }

            alternate_position = alternate_next_position;
        }
        }
      

        position = next_position;
    }

    possible_obstruction_positions.len().to_string()
}

// Parse the input, returning
// - The shape of the map
// - Location of any obstructions
// - The starting position
// - The starting direction
pub fn parse_map(
    input: &str,
) -> (
    (usize, usize),
    HashSet<(usize, usize)>,
    (usize, usize),
    (i32, i32),
) {
    let (mut m, mut n) = (0, 0);
    let mut obstructions: HashSet<(usize, usize)> = HashSet::new();
    let mut position: (usize, usize) = (0, 0);
    let mut direction: (i32, i32) = (0, 1);

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

    ((m, n), obstructions, position, direction)
}

fn turn_right(direction: (i32, i32)) -> (i32, i32) {
    match direction {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => unreachable!(),
    }
}
