use std::{collections::HashMap, task::Wake};

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
        'T' => 10,
        'J' => 1,
        other => String::from(other).parse::<u32>().unwrap(),
    }
}

fn get_strength(cards: &str) -> Strength {
    let mut chars: HashMap<char, u32> = HashMap::new();
    for c in cards.chars() {
        *chars.entry(c).or_insert(0) += 1
    }

    let mut distinct_count = chars.len() as u32;

    let jokers: u32 = *chars.get(&'J').unwrap_or(&0);
    if jokers > 0 {
        chars.remove(&'J');
    }

    let values: Vec<&u32> = chars.values().collect::<Vec<&u32>>();
    let mut max = **values.iter().max().unwrap_or(&&0);

    if jokers > 0 {
        distinct_count -= 1;
        max += jokers;
    }

    if max >= 5 {
        return if distinct_count <= 1 {
            Strength::FiveOfAKind
        } else {
            Strength::FourOfAKind
        };
    }

    if max == 4 {
        return Strength::FourOfAKind;
    }

    if max == 3 {
        return if distinct_count == 2 {
            Strength::FullHouse
        } else {
            Strength::ThreeOfAKind
        };
    }

    if max == 2 {
        return if distinct_count == 3 {
            Strength::TwoPair
        } else {
            Strength::OnePair
        };
    }

    Strength::HighCard
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
    let input = include_str!("../input.txt");
    //     let input = "32T3K 765
    // T55J5 684
    // KK677 28
    // KTJJT 220
    // QQQJA 483";

    let mut hands: Vec<Hand> = input
        .lines()
        .map(make_hand_from_line)
        .collect::<Vec<Hand>>();

    hands.sort();

    let mut total_winnings = 0;
    for (rank, hand) in hands.iter().enumerate() {
        total_winnings += (rank as u32 + 1) * hand.bid;
    }

    println!("Total winnings: {}", total_winnings);
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
        assert_eq!(get_strength("AKQ5T"), Strength::HighCard);
    }

    #[test]
    fn test_get_strength_with_jokers() {
        assert_eq!(get_strength("JJJJA"), Strength::FiveOfAKind);
        assert_eq!(get_strength("AAAJ1"), Strength::FourOfAKind);
        assert_eq!(get_strength("AAJ31"), Strength::ThreeOfAKind);
        assert_eq!(get_strength("AAAQQ"), Strength::FullHouse);
        assert_eq!(get_strength("AJQKK"), Strength::ThreeOfAKind);
        assert_eq!(get_strength("KJ463"), Strength::OnePair);
        assert_eq!(get_strength("AKQ5T"), Strength::HighCard);
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
    fn test_equivalence_with_joker() {
        assert_eq!(get_card_value('J'), 1);
    }

    #[test]
    fn test_sorting_with_joker() {
        let mut items: [Hand; 2] = [
            make_hand_from_line("QQQ2J 1"),
            make_hand_from_line("QQQJA 2"),
        ];

        items.sort();

        assert_eq!(items[0].bid, 2);
        assert_eq!(items[1].bid, 1);
    }

    #[test]
    fn test_more_sorting_with_jokers() {
        let mut items: [Hand; 3] = [
            make_hand_from_line("KTJJT 1"),
            make_hand_from_line("T55J5 10"),
            make_hand_from_line("QQQJA 100"),
        ];

        items.sort();

        assert_eq!(items[0].bid, 10);
        assert_eq!(items[1].bid, 100);
        assert_eq!(items[2].bid, 1);
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

    #[test]
    fn test_five_of_a_kind() {
        assert_eq!(get_strength("AAAAA"), Strength::FiveOfAKind);
        assert_eq!(get_strength("AAAAJ"), Strength::FiveOfAKind);
        assert_eq!(get_strength("JAAAJ"), Strength::FiveOfAKind);
        assert_eq!(get_strength("JJAAJ"), Strength::FiveOfAKind);
        assert_eq!(get_strength("JJJAJ"), Strength::FiveOfAKind);
        assert_eq!(get_strength("JJJJJ"), Strength::FiveOfAKind);
    }

    #[test]
    fn test_four_of_a_kind() {
        assert_eq!(get_strength("AAAAQ"), Strength::FourOfAKind);
        assert_eq!(get_strength("JAAAQ"), Strength::FourOfAKind);
        assert_eq!(get_strength("JJAAQ"), Strength::FourOfAKind);
        assert_eq!(get_strength("JJJAQ"), Strength::FourOfAKind);
    }

    #[test]
    fn test_full_house() {
        assert_eq!(get_strength("AAAQQ"), Strength::FullHouse);
        assert_eq!(get_strength("AAQQQ"), Strength::FullHouse);
        assert_eq!(get_strength("JAAQQ"), Strength::FullHouse);
        assert_eq!(get_strength("AAQQJ"), Strength::FullHouse);
    }

    #[test]
    fn test_three_of_a_kind() {
        assert_eq!(get_strength("AAAQK"), Strength::ThreeOfAKind);
        assert_eq!(get_strength("AAJQK"), Strength::ThreeOfAKind);
        assert_eq!(get_strength("JAJQK"), Strength::ThreeOfAKind);
    }
}
