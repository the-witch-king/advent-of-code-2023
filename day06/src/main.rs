use regex::Regex;

fn get_number_from_line(line: &str) -> u64 {
    Regex::new(r"[^\d]")
        .unwrap()
        .replace_all(line, "")
        .parse::<u64>()
        .unwrap_or(0)
}

fn get_numbers_from_line(line: &str) -> Vec<u64> {
    Regex::new(r"\d+")
        .unwrap()
        .captures_iter(line)
        .map(|m| m[0].parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn main() {
    let input = include_str!("../input.txt");
    //     let input = "Time:      7  15   30
    // Distance:  9  40  200";

    // Pt 1
    let mut lines = input.lines();
    let times = get_numbers_from_line(lines.next().unwrap());
    let distances = get_numbers_from_line(lines.next().unwrap());

    println!(
        "Part 1 Ways to win: {}",
        get_product_of_ways_to_win(times, distances)
    );

    // Pt 2
    let mut lines = input.lines();
    let time = get_number_from_line(lines.next().unwrap());
    let distance = get_number_from_line(lines.next().unwrap());
    println!(
        "Part 2 ways to win: {}",
        get_amount_of_ways_to_win(time, distance)
    );
}

fn get_product_of_ways_to_win(times: Vec<u64>, distances: Vec<u64>) -> u64 {
    let mut ways = 1;
    for i in 0..times.len() {
        ways *= get_amount_of_ways_to_win(times[i], distances[i])
    }
    ways
}

fn get_amount_of_ways_to_win(time: u64, distance: u64) -> u64 {
    let mut ways = 0;

    for i in 0..time {
        let speed = i;
        let remaining_time = time - speed;
        let distance_traveled = remaining_time * speed;
        if distance_traveled > distance {
            ways += 1;
        }
    }

    ways
}
