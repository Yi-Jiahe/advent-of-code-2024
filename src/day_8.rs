use std::collections::{HashMap, HashSet};

pub fn part_1(input: &str) -> String {
    let ((m, n), antennas) = parse_map(input);

    let mut valid_antinodes: HashSet<(usize, usize)> = HashSet::new();

    for (_frequency, positions) in antennas.iter() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];

                let v = (x2 as i32 - x1 as i32, y2 as i32 - y1 as i32);

                let a = (x2 as i32 + v.0, y2 as i32 + v.1);
                let b = (x1 as i32 - v.0, y1 as i32 - v.1);

                for antinode in [a, b] {
                    if antinode.0 < 0
                        || antinode.0 >= n as i32
                        || antinode.1 < 0
                        || antinode.1 >= m as i32
                    {
                        continue;
                    }

                    valid_antinodes.insert((antinode.0 as usize, antinode.1 as usize));
                }
            }
        }
    }
    valid_antinodes.len().to_string()
}

pub fn part_2(input: &str) -> String {
    todo!()
}

fn parse_map(input: &str) -> ((usize, usize), HashMap<char, Vec<(usize, usize)>>) {
    let (mut m, mut n) = (0, 0);
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            n = line.trim().len();
        }

        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_insert(Vec::new()).push((j, i));
            }
        }

        m = i + 1;
    }

    ((m, n), antennas)
}
