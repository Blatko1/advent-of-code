use std::cmp::Ordering;

fn main() {
    let input = include_str!("../inputs/07.txt");
    let lines = input.lines().map(|l| {
        let input: Vec<&str> = l.split_whitespace().collect();
        (input[0], input[1].parse::<u32>().unwrap())
    });

    let mut hands = Vec::new();
    lines.for_each(|(hand_str, bid)| {
        let hand = Hand::from(hand_str);
        hands.push((hand, bid))
    });
    hands.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    let mut sum = 0;
    for (f, &(_, bid)) in hands.iter().enumerate() {
        let factor = f as u32 + 1;
        sum += bid * factor;
    }
    println!("sum: {}", sum);
}

#[derive(Debug, PartialEq, Eq)]
struct Hand([Card; 5]);

impl Hand {
    // The better solution from:
    // https://github.com/tlent/advent-of-code/blob/main/year_2023/day_07/src/lib.rs
    fn get_type(&self) -> HandType {
        let mut card_counts = [0; 14];
        let mut max_count = 0;
        let mut second_max_count = 0;
        for &card in self.0.iter() {
            let count = &mut card_counts[card as usize];
            *count += 1;
            if card == Card::Joker {
                continue;
            }
            if *count > max_count {
                max_count = *count;
            } else if *count > second_max_count {
                second_max_count = *count;
            }
        }
        max_count += card_counts[Card::Joker as usize];
        match (max_count, second_max_count) {
            (5, _) => HandType::FiveOfKind,
            (4, _) => HandType::FourOfKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfKind,
            (2, 2) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let hand1 = self.get_type();
        let hand2 = other.get_type();

        match hand1.cmp(&hand2) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Equal => {
                for (a, b) in self.0.iter().zip(other.0.iter()) {
                    match a.cmp(b) {
                        Ordering::Less => return Some(Ordering::Less),
                        Ordering::Greater => return Some(Ordering::Greater),
                        Ordering::Equal => (),
                    }
                }
                unreachable!("Two cards are the same: {:?} {:?}", self, other)
            }
        }
    }
}

// The hand types are ordered from the smallest to the biggest
// value as written here.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();
        assert_eq!(chars.clone().count(), 5);
        let hand = [
            Card::from(chars.next().unwrap()),
            Card::from(chars.next().unwrap()),
            Card::from(chars.next().unwrap()),
            Card::from(chars.next().unwrap()),
            Card::from(chars.next().unwrap()),
        ];
        Self(hand)
    }
}

// The card types are ordered from the smallest to the biggest
// value as written here.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Joker,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => unreachable!("Unknown card: {}", value),
        }
    }
}

#[test]
fn card_type_test() {
    assert_eq!(Hand::from("QJJQ2").get_type(), HandType::FourOfKind);
    assert_eq!(Hand::from("JKKK2").get_type(), HandType::FourOfKind);
    assert!(Hand::from("QJJQ2") > Hand::from("JKKK2"));
    assert!(Hand::from("QJJQ2") < Hand::from("KKKK2"));
    assert!(Hand::from("JQJQ2") < Hand::from("JKKK2"));
    assert!(Hand::from("JQJQ2") > Hand::from("JKK22"));

    assert!(HandType::FourOfKind < HandType::FiveOfKind);
    assert!(HandType::OnePair > HandType::HighCard);
}
