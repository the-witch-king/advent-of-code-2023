use regex::Regex;

fn main() {
    let lines: Vec<&str> = include_str!("../../input.txt").lines().collect();
    let mut sum: u32 = 0;

    for line in lines {
        let left = get_first_digit(line);
        let right = get_last_digit(line);

        let left_digit = get_digit_from_string(&left);
        let right_digit = get_digit_from_string(&right);

        let result = left_digit.to_owned() + right_digit;
        sum += result.parse::<u32>().unwrap();
    }

    println!("Sum: {}", sum);
}

fn get_first_digit(x: &str) -> String {
    let regex_string = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        r"\d", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    );

    let regex = Regex::new(&regex_string).unwrap();

    return regex.find(x).unwrap().as_str().to_string();
}

fn get_last_digit(x: &str) -> String {
    let regex_string = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        r"\d", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"
    );

    let regex = Regex::new(&regex_string).unwrap();

    let reversed_line: String = reverse_string(x);

    let reversed_number = regex.find(&reversed_line).unwrap().as_str().to_string();
    reverse_string(&reversed_number)
}

fn reverse_string(x: &str) -> String {
    let mut reversed_chars: Vec<char> = x.chars().collect();
    reversed_chars.reverse();
    reversed_chars.into_iter().collect()
}

fn get_digit_from_string(x: &str) -> &str {
    match x {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => x,
    }
}
