use regex::Regex;

fn get_numbers_from_line(line: &str) -> Vec<u32> {
    Regex::new(r"\d+")
        .unwrap()
        .captures_iter(line)
        .map(|m| m[0].parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn main() {
    let input = include_str!("../input.txt");
    //     let input = "Time:      7  15   30
    // Distance:  9  40  200";

    let mut lines = input.lines();
    let times = get_numbers_from_line(lines.next().unwrap());
    let distances = get_numbers_from_line(lines.next().unwrap());

    // Pt 1
    println!(
        "Ways to win: {}",
        get_product_of_ways_to_win(times, distances)
    );
}

fn get_product_of_ways_to_win(times: Vec<u32>, distances: Vec<u32>) -> u32 {
    let mut ways = 1;
    for i in 0..times.len() {
        ways *= get_amount_of_ways_to_win(times[i], distances[i])
    }
    ways
}

fn get_amount_of_ways_to_win(time: u32, distance: u32) -> u32 {
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
