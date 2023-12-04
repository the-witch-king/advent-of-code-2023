use regex::Regex;

fn main() {
    let digit_regex = Regex::new(r"\d").unwrap();
    let lines: Vec<&str> = include_str!("../../input.txt").lines().collect();
    let mut sum: u32 = 0;

    for line in lines {
        let digits: Vec<&str> = digit_regex
            .find_iter(line)
            .map(|digit| digit.as_str())
            .collect();

        let result = digits[0].to_owned() + digits[digits.len() - 1];
        sum += result.parse::<u32>().unwrap();
    }

    println!("Sum: {}", sum);
}
