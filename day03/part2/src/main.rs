use std::collections::HashSet;

use regex::Regex;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Part {
    value: u32,
    row: usize,
    start: usize,
    end: usize,
}

type PartsMap = Vec<Vec<Part>>;

fn get_gear_ratio(x: usize, y: usize, parts_map: &PartsMap) -> u32 {
    let to_check: Vec<(usize, usize)> = vec![
        (x - 1, y - 1), // TL
        (x, y - 1),     // T
        (x + 1, y - 1), // TR
        (x - 1, y),     // L
        (x + 1, y),     // R
        (x - 1, y + 1), // BL
        (x, y + 1),     // B
        (x + 1, y + 1), // BR
    ];
    let mut parts: HashSet<Part> = HashSet::new();

    for check in to_check {
        let (x0, y0) = check;
        let part = parts_map.get(y0).unwrap().get(x0).unwrap();
        if part.value != 0 {
            parts.insert(part.clone());
        }
    }

    if parts.len() != 2 {
        return 0;
    }

    parts.iter().map(|p| p.value).product::<u32>()
}

fn main() {
    let input = include_str!("../../input.txt");

    let amount_of_rows = input.lines().count();
    let line_length = input.lines().next().unwrap().len();
    let mut parts_map: PartsMap = Vec::new();
    for _ in 0..amount_of_rows {
        let mut row: Vec<Part> = Vec::new();
        for _ in 0..line_length {
            row.push(Part {
                value: 0,
                row: 0,
                start: 0,
                end: 0,
            });
        }

        parts_map.push(row);
    }

    let regex = Regex::new(r"(?m)\d+").unwrap();
    for (row, line) in input.lines().enumerate() {
        for capture in regex.captures_iter(line) {
            let c = capture.get(0).unwrap();
            let start = c.start();
            let end = c.end() - 1;
            let value = c.as_str().parse::<u32>().unwrap();

            for i in start..=end {
                parts_map[row][i] = Part {
                    value,
                    row,
                    start,
                    end,
                };
            }
        }
    }

    let mut sum_of_ratios: u32 = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '*' {
                sum_of_ratios += get_gear_ratio(x, y, &parts_map);
            }
        }
    }

    println!("Sum: {}", sum_of_ratios);
}
