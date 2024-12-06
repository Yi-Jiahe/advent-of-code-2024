pub fn part_1(input: &str) -> String {
    let mut word_search: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        word_search.push(line.chars().collect::<Vec<char>>());
    }

    let m = word_search.len();
    // The word search is a rectangle
    let n = word_search[0].len();

    dbg!(m, n);

    let mut appearances = 0;

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let search_word = "XMAS".chars().collect::<Vec<char>>();

    for i in 0..m {
        for j in 0..n {
            for direction in directions {
                // Don't bother searching in directions in which the word won't fit
                let end_x = j as i32 + (search_word.len() - 1) as i32 * direction.0;
                let end_y = i as i32 + (search_word.len() - 1) as i32 * direction.1;
                if end_x < 0 || end_x >= n as i32 || end_y < 0 || end_y >= m as i32 {
                    continue;
                }

                for k in 0..search_word.len() {
                    let x = (j as i32 + k as i32 * direction.0) as usize;
                    let y = (i as i32 + k as i32 * direction.1) as usize;
                    if word_search[y][x] != search_word[k] {
                        break;
                    }
                    if k == search_word.len() - 1 {
                        // dbg!(i, j, direction);
                        appearances += 1;
                    }
                }
            }
        }
    }

    appearances.to_string()
}

pub fn part_2(input: &str) -> String {
    todo!()
}
