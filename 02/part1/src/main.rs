const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let lines: Vec<&str> = include_str!("../../input.txt").lines().collect();
    let games = lines
        .iter()
        .map(|&line| parse_game_line(line))
        .collect::<Vec<Game>>();

    let mut sum: u32 = 0;
    for game in games {
        let mut possible = true;
        for round in game.rounds {
            if round.blue > MAX_BLUE || round.green > MAX_GREEN || round.red > MAX_RED {
                possible = false;
            }
        }

        if possible {
            sum += game.id;
        }
    }

    println!("Sum: {}", sum);
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
    // [3 blue, 4 red]
    let mut round = Round {
        red: 0,
        green: 0,
        blue: 0,
    };
    for part in parts {
        let p = part.trim().split(' ').collect::<Vec<&str>>();
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
