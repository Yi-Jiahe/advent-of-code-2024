use std::{env, fs, path::Path};

use advent_of_code_2024::{day_1, day_2, day_3};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let day = &args[1];

    let path = Path::new(".").join("input").join(format!("day_{}.txt", day));
    dbg!(&path);

    let input = fs::read_to_string(path).unwrap();
    // dbg!(&input);

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
        _ => todo!(),
    };
}
