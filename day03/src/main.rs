use regex::Regex;

fn is_symbol(c: char) -> bool {
    match c {
        c if c.is_ascii_digit() => false,
        c if c.is_ascii_alphabetic() => false,
        '.' => false,
        _ => true,
    }
}

fn is_known_part(start: usize, end: usize, rows: Vec<&str>) -> bool {
    let left_index = if start == 0 { 0 } else { start - 1 };
    let max_line_length = rows[0].len();
    let right_index = if end == max_line_length - 1 {
        max_line_length - 1
    } else {
        end + 1
    };

    for row in rows.into_iter() {
        for (i, c) in row.chars().enumerate() {
            if i < left_index || i > right_index {
                continue;
            }

            if is_symbol(c) {
                return true;
            }
        }
    }

    false
}

fn main() {
    let input = include_str!("../input.txt");
    //     let input = "467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..";

    let regex = Regex::new(r"(?m)\d+").unwrap();
    let mut ids: Vec<u32> = Vec::new();

    for (row, line) in input.lines().enumerate() {
        let skip = if row < 2 { 0 } else { row - 1 };
        let take = if row == 0 { 2 } else { 3 };
        let rows_to_check = input.lines().skip(skip).take(take).collect::<Vec<&str>>();

        let captures = regex.captures_iter(line);
        for capture in captures {
            let c = capture.get(0).unwrap();
            let start = c.start();
            let end = c.end() - 1;

            if is_known_part(start, end, rows_to_check.clone()) {
                ids.push(capture.get(0).unwrap().as_str().parse::<u32>().unwrap());
            }
        }
    }

    println!("Sum: {}", ids.iter().sum::<u32>());
}
