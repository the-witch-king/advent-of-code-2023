const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_game_line(game_line: &str) -> Game {
    let parts = game_line.split(':').collect::<Vec<&str>>();
    let game = parts[0].split(' ').collect::<Vec<&str>>();
    let id = game[1].parse::<u32>().unwrap();

    let game_rounds = parts[1].split(';').collect::<Vec<&str>>();
    let rounds = game_rounds
        .iter()
        .map(|&r| parse_round(r))
        .collect::<Vec<Round>>();

    Game { id, rounds }
}

fn parse_round(round_line: &str) -> Round {
    let parts = round_line.split(',').collect::<Vec<&str>>();
    let mut round = Round {
        red: 0,
        green: 0,
        blue: 0,
    };
    for part in parts {
t       let p = part.trim().split(' ').collect::<Vec<&str>>();
        let amount = p[0].parse::<u32>().unwrap();
        let color = p[1];

        match color {
            "red" => round.red = amount,
            "green" => round.green = amount,
            "blue" => round.blue = amount,
            _ => panic!("Invalid color"),
        }
    }

    round
}

fn main() {
    let lines: Vec<&str> = include_str!("../input.txt").lines().collect();
    let games = lines
        .iter()
        .map(|&line| parse_game_line(line))
        .collect::<Vec<Game>>();

    // Part 1
    print_sum_of_possible_games(&games);

    // Part 2
    print_sum_of_power_cubes(&games);
}

/**
 * Part 1 solution here
 */
fn print_sum_of_possible_games(games: &[Game]) {
    let mut sum: u32 = 0;
    for game in games {
        if !game
            .rounds
            .iter()
            .any(|round| round.blue > MAX_BLUE || round.green > MAX_GREEN || round.red > MAX_RED)
        {
            sum += game.id;
        }
    }

    println!("Part 1: Sum is {}", sum);
}

/**
 * Part 2 solution here
 */
fn print_sum_of_power_cubes(games: &[Game]) {
    let sum = games.iter().map(get_game_power_of_cubes).sum::<u32>();
    println!("Part 2: Sum is {}", sum);
}

fn get_game_power_of_cubes(game: &Game) -> u32 {
    let mut min_blue: u32 = 0;
    let mut min_green: u32 = 0;
    let mut min_red: u32 = 0;

    for round in game.rounds.iter() {
        if round.green > min_green {
            min_green = round.green;
        }
        if round.blue > min_blue {
            min_blue = round.blue;
        }
        if round.red > min_red {
            min_red = round.red;
        }
    }

    min_blue * min_green * min_red
}
