#[derive(Debug, PartialEq)]
enum SchematicType {
    Lock,
    Key,
}

pub fn part_1(input: &str) -> String {
    let (locks, keys) = parse_schematics(input);

    let mut ans = 0;

    for lock in locks {
        'keys_loop: for key in &keys {
            for i in 0..5 {
                if lock[i] + key[i] > 5 {
                    continue 'keys_loop;
                }
            }
            ans += 1;
        }
    }

    ans.to_string()
}

pub fn part_2(input: &str) -> String {
    unimplemented!()
}

fn init_heights() -> [Vec<char>; 5] {
  [vec![], vec![], vec![], vec![], vec![]]
}

fn parse_schematics(input: &str) -> (Vec<[u32; 5]>, Vec<[u32; 5]>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let mut schematic_type: Option<SchematicType> = None;
    let mut heights_char = init_heights();
    let mut rows = 0;
    // Add empty row to finish parsing last schematic
    for line in format!("{}\n\n", input).lines() {
        if schematic_type.is_none() {
            if line == "....." {
                schematic_type = Some(SchematicType::Key);
            } else if line == "#####" {
                schematic_type = Some(SchematicType::Lock);
            } else {
                panic!("Invalid schematic type");
            }
            continue;
        }

        if line.is_empty() {
            let mut heights = [0, 0, 0, 0, 0];
            for (i, v) in heights_char.iter().enumerate() {
                for c in v {
                    if *c == '#' {
                        heights[i] += 1;
                    } else if *c == '.' {
                        continue;
                    } else {
                        panic!("Invalid character");
                    }
                }
            }

            match schematic_type {
                None => panic!("Invalid schematic type"),
                Some(SchematicType::Lock) => locks.push(heights),
                Some(SchematicType::Key) => keys.push(heights),
            }

            // Reset
            schematic_type = None;
            heights_char = init_heights();
            rows = 0;
            continue;
        }

        for (i, c) in line.chars().enumerate() {
            // Skip bottom row
            if rows >= 5 {
                continue;
            }
            match schematic_type {
                None => panic!("Invalid schematic type"),
                Some(SchematicType::Lock) => heights_char[i].push(c),
                Some(SchematicType::Key) => heights_char[i].insert(0, c),
            }
        }
        rows += 1;
    }

    (locks, keys)
}
