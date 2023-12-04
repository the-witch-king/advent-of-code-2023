use regex::Regex;

#[derive(Debug)]
struct Card {
    id: String,
    wins: u32,
    player_numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

fn main() {
    let input = include_str!("../../input.txt");
    //     let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let mut cards: Vec<Card> = Vec::new();
    let mut card_amounts: Vec<u32> = Vec::new();

    for line in input.lines() {
        let parts = line.split(':').collect::<Vec<&str>>();
        let id = parts[0].trim();

        let numbers = parts[1].split('|').collect::<Vec<&str>>();

        let regex = Regex::new(r"(?m)\d+").unwrap();
        let player_numbers = regex
            .captures_iter(numbers[0])
            .map(|c| c[0].parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let winning_numbers = regex
            .captures_iter(numbers[1])
            .map(|c| c[0].parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mut wins = 0;
        for pn in player_numbers.iter() {
            if winning_numbers.contains(pn) {
                wins += 1
            }
        }

        let card = Card {
            id: id.to_string(),
            wins,
            player_numbers,
            winning_numbers,
        };

        cards.push(card);
        card_amounts.push(1);
    }

    let length = cards.len();
    for id in 0..length {
        let card = &cards[id];

        let until = card.wins;
        let amount = card_amounts[id];

        for _ in 0..amount {
            for i in id..(id + (until as usize)) {
                let index = i + 1;
                if index >= length {
                    break;
                }

                card_amounts[index] += 1;
            }
        }
    }

    println!("Total sum: {}", card_amounts.iter().sum::<u32>());
}
