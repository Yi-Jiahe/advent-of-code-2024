use std::collections::HashMap;

pub fn part_1(input: &str) -> String {
    let mut stones = input
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut n_blinks = 0;
    while n_blinks < 25 {
        blink(&mut stones);
        n_blinks += 1;
    }

    stones.len().to_string()
}

pub fn part_2(input: &str) -> String {
    let stones = input
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut stones_map: HashMap<u64, u64> = HashMap::new();
    for stone in stones {
        if !stones_map.contains_key(&stone) {
            stones_map.insert(stone, 0);
        }
        stones_map.insert(stone, stones_map[&stone] + 1);
    }

    let transformations_map = HashMap::from([(0, vec![1])]);

    let mut n_blinks = 0;
    while n_blinks < 75 {
        let now = std::time::Instant::now();

        let mut next_stones_map: HashMap<u64, u64> = HashMap::new();

        for (stone, count) in stones_map.iter() {
            let new_stones = match transformations_map.get(stone) {
                Some(new_stones) => new_stones.clone(),
                None => {
                    // 0 case is already hardcoded into transformations map
                    if stone.to_string().len() % 2 == 0 {
                      
                        let stone_str = stone.to_string();
                        let len = stone_str.len();
let left = &stone_str[0..len / 2].parse::<u64>().unwrap();
            let right = &stone_str[len / 2..len].parse::<u64>().unwrap();
vec![*left, *right]
                    } else {
                      vec![*stone * 2024]
                    }
                }
            };

            for new_stone in new_stones {
                if !next_stones_map.contains_key(&new_stone) {
                    next_stones_map.insert(new_stone, 0);
                }
                next_stones_map.insert(new_stone, next_stones_map[&new_stone] + count);
            }
        }
        stones_map = next_stones_map;

        n_blinks += 1;
        dbg!(n_blinks, now.elapsed().as_millis());
    }

    let ans = stones_map.iter().fold(0, |acc, (_, v)| acc + v);
    ans.to_string()
}

fn blink(stones: &mut Vec<u64>) {
    let mut i = 0;
    while i < stones.len() {
        let stone = stones[i];

        if stone == 0 {
            stones[i] = 1;

            i += 1;
        } else if stone.to_string().len() % 2 == 0 {
            let stone_str = stone.to_string();
            let len = stone_str.len();
            let left = &stone_str[0..len / 2].parse::<u64>().unwrap();
            let right = &stone_str[len / 2..len].parse::<u64>().unwrap();

            stones.remove(i);
            stones.insert(i, *right);
            stones.insert(i, *left);

            i += 2;
        } else {
            stones[i] = stone * 2024;

            i += 1;
        }
    }
}
