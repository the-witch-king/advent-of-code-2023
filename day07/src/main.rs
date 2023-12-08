use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Strength {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u32,
    strength: Strength,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.strength == other.strength {
            let mut self_chars = self.cards.chars();
            let mut other_chars = other.cards.chars();
            while let (Some(sc), Some(oc)) = (self_chars.next(), other_chars.next()) {
                if sc == oc {
                    continue;
                } else {
                    let sc_val = get_card_value(sc);
                    let oc_val = get_card_value(oc);
                    return sc_val.cmp(&oc_val);
                }
            }
        }
        self.strength.cmp(&other.strength)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_card_value(card: char) -> u32 {
    match card {
        'A' => 100,
        'K' => 90,
        'Q' => 80,
        'J' => 70,
        'T' => 10,
        other => String::from(other).parse::<u32>().unwrap(),
    }
}

fn get_strength(cards: &str) -> Strength {
    let mut chars: HashMap<char, u32> = HashMap::new();
    for c in cards.chars() {
        *chars.entry(c).or_insert(0) += 1
    }

    let values: Vec<&u32> = chars.values().collect::<Vec<&u32>>();
    let length = values.len();

    match length {
        1 => Strength::FiveOfAKind,
        2 => {
            if **values.iter().max().unwrap() == 4 {
                Strength::FourOfAKind
            } else {
                Strength::FullHouse
            }
        }
        3 => {
            if **values.iter().max().unwrap() == 3 {
                Strength::ThreeOfAKind
            } else {
                Strength::TwoPair
            }
        }
        4 => Strength::OnePair,
        _ => Strength::HighCard,
    }
}

fn make_hand_from_line(l: &str) -> Hand {
    let parts = l.split(' ').collect::<Vec<&str>>();
    let cards = String::from(parts[0]);
    let bid = parts[1].parse::<u32>().unwrap();

    Hand {
        strength: get_strength(&cards),
        cards,
        bid,
    }
}

fn main() {
    // let input = include_str!("../input.txt");
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    let hands: Vec<Hand> = input
        .lines()
        .map(make_hand_from_line)
        .collect::<Vec<Hand>>();

    println!("Cards: {:?}", hands);
    println!("They are equal? {}", hands[0] == hands[0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strengths() {
        assert_eq!(get_strength("AAAAA"), Strength::FiveOfAKind);
        assert_eq!(get_strength("AAAA1"), Strength::FourOfAKind);
        assert_eq!(get_strength("AAA31"), Strength::ThreeOfAKind);
        assert_eq!(get_strength("AAAQQ"), Strength::FullHouse);
        assert_eq!(get_strength("AAQQK"), Strength::TwoPair);
        assert_eq!(get_strength("KK463"), Strength::OnePair);
        assert_eq!(get_strength("AKQJT"), Strength::HighCard);
    }

    #[test]
    fn test_equalivents() {
        let hand_1 = Hand {
            strength: Strength::HighCard,
            cards: String::from("AKQJT"),
            bid: 0,
        };
        let hand_2 = Hand {
            strength: Strength::HighCard,
            cards: String::from("AKQJT"),
            bid: 0,
        };

        assert!(hand_1 >= hand_2);
        assert!(hand_1 <= hand_2);
        assert!(hand_1 == hand_2);
    }

    #[test]
    fn test_sorting_by_highest_card() {
        let mut hands = [
            Hand {
                strength: Strength::HighCard,
                cards: String::from("AAAA3"),
                bid: 0,
            },
            Hand {
                strength: Strength::HighCard,
                cards: String::from("AAAA2"),
                bid: 0,
            },
            Hand {
                strength: Strength::HighCard,
                cards: String::from("AAAAQ"),
                bid: 0,
            },
            Hand {
                strength: Strength::HighCard,
                cards: String::from("AAAAA"),
                bid: 0,
            },
        ];

        hands.sort();

        assert_eq!(hands[0].cards, "AAAA2");
        assert_eq!(hands[1].cards, "AAAA3");
        assert_eq!(hands[2].cards, "AAAAQ");
        assert_eq!(hands[3].cards, "AAAAA");
    }

    #[test]
    fn test_sorting() {
        let mut hands = [
            Hand {
                strength: Strength::HighCard,
                cards: String::from("4567A"),
                bid: 0,
            },
            Hand {
                strength: Strength::HighCard,
                cards: String::from("4567Q"),
                bid: 0,
            },
            Hand {
                strength: Strength::FullHouse,
                cards: String::from("AAAQQ"),
                bid: 0,
            },
            Hand {
                strength: Strength::FourOfAKind,
                cards: String::from("AAAA5"),
                bid: 0,
            },
            Hand {
                strength: Strength::FiveOfAKind,
                cards: String::from("AAAAA"),
                bid: 0,
            },
        ];

        hands.sort();

        assert_eq!(hands[0].cards, "4567Q");
        assert_eq!(hands[1].cards, "4567A");
        assert_eq!(hands[2].cards, "AAAQQ");
        assert_eq!(hands[3].cards, "AAAA5");
        assert_eq!(hands[4].cards, "AAAAA");
    }
}
