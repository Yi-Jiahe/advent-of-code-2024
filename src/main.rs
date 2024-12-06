use std::{env, fs, path::Path};

use advent_of_code_2024::{day_1, day_2, day_3, day_4, day_5, day_6};

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];

    let path = Path::new(".").join("input").join(format!("day_{}.txt", day));

    let input = fs::read_to_string(path).unwrap();

    match day.as_str() {
        "1" => {
            let ans_part_1 = day_1::part_1(&input);
            println!("Part 1: {}", ans_part_1);

            let ans_part_2 = day_1::part_2(&input);
            println!("Part 2: {}", ans_part_2);
        },
        "2" => {
            let ans_part_1 = day_2::part_1(&input);
            println!("Part 1: {}", ans_part_1);

            let ans_part_2 = day_2::part_2(&input);
            println!("Part 2: {}", ans_part_2);
        }
        "3" => {
            let ans_part_1 = day_3::part_1(&input);
            println!("Part 1: {}", ans_part_1);

            let ans_part_2 = day_3::part_2(&input);
            println!("Part 2: {}", ans_part_2);
        }
        "4" => {
            let ans_part_1 = day_4::part_1(&input);
            println!("Part 1: {}", ans_part_1);

            let ans_part_2 = day_4::part_2(&input);
            println!("Part 2: {}", ans_part_2);
        }
        "5" => {
            let ans_part_1 = day_5::part_1(&input);
            println!("Part 1: {}", ans_part_1);

            let ans_part_2 = day_5::part_2(&input);
            println!("Part 2: {}", ans_part_2);
        }
        "6" => {
            let ans_part_1 = day_6::part_1(&input);
            println!("Part 1: {}", ans_part_1);

            let ans_part_2 = day_6::part_2(&input);
            println!("Part 2: {}", ans_part_2);
        }
        _ => todo!(),
    };
}
