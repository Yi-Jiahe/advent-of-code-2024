pub fn part_1(input: &str) -> String {
    let mut word_search: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        word_search.push(line.chars().collect::<Vec<char>>());
    }

    let m = word_search.len();
    // The word search is a rectangle
    let n = word_search[0].len();

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
    let mut word_search: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        word_search.push(line.chars().collect::<Vec<char>>());
    }

    let m = word_search.len();
    // The word search is a rectangle
    let n = word_search[0].len();

    let up_left = (-1, -1);
    let down_right = (1, 1);
    let down_left = (-1, 1);
    let up_right = (1, -1);

    let mut appearances: i32 = 0;

    for i in 1..m - 1 {
        for j in 1..n - 1 {
            if word_search[i][j] != 'A' {
                continue;
            }

            let top_left = ((j as i32 + up_left.0) as usize, (i as i32 + up_left.1) as usize);
            let bottom_right = ((j as i32 + down_right.0) as usize, (i as i32 + down_right.1) as usize);
            let bottom_left = ((j as i32 + down_left.0) as usize, (i as i32 + down_left.1) as usize);
            let top_right = ((j as i32 + up_right.0) as usize, (i as i32 + up_right.1) as usize);

            if ((word_search[top_left.1][top_left.0] == 'M'
                && word_search[bottom_right.1][bottom_right.0] == 'S')
                || (word_search[top_left.1][top_left.0] == 'S'
                    && word_search[bottom_right.1][bottom_right.0] == 'M'))
                && ((word_search[bottom_left.1][bottom_left.0] == 'M'
                    && word_search[top_right.1][top_right.0] == 'S')
                    || (word_search[bottom_left.1][bottom_left.0] == 'S'
                        && word_search[top_right.1][top_right.0] == 'M'))
            {
              appearances += 1;
            }
        }
    }

    appearances.to_string()
}
