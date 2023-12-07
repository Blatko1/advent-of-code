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
    fn get_type(&self) -> HandType {
        let mut hand = self.0.clone();
        hand.sort();
        let elem2 = hand[1];
        let elem4 = hand[3];

        // Check for a Five of a kind
        // Possibilities: AAAAA
        if hand[0] == hand[4] {
            return HandType::FiveOfKind;
        }

        // Check for a Four of a kind
        // Possibilities: AAAAx, xAAAA
        if hand[0..=3].iter().all(|&elem| elem == elem2) {
            return HandType::FourOfKind;
        } else if hand[1..=4].iter().all(|&elem| elem == elem2) {
            return HandType::FourOfKind;
        }

        // Check for a Full house
        // Possibilities: AAABB, AABBB
        if hand[0..=2].iter().all(|&elem| elem == elem2)
            && hand[3..=4].iter().all(|&elem| elem == elem4)
        {
            return HandType::FullHouse;
        } else if hand[0..=1].iter().all(|&elem| elem == elem2)
            && hand[2..=4].iter().all(|&elem| elem == elem4)
        {
            return HandType::FullHouse;
        }

        // Check for a Three of a kind
        // Possibilities: AAAxy, xAAAy, xyAAA
        if hand[0..=2].iter().all(|&elem| elem == elem2)
            || hand[1..=3].iter().all(|&elem| elem == elem2)
            || hand[2..=4].iter().all(|&elem| elem == elem4)
        {
            return HandType::ThreeOfKind;
        }

        // Check for a Two pair
        // Possibilities: AABBx, AAxBB, xAABB
        if hand[0..=1].iter().all(|&elem| elem == elem2) {
            if hand[2..=3].iter().all(|&elem| elem == elem4)
                || hand[3..=4].iter().all(|&elem| elem == elem4)
            {
                return HandType::TwoPair;
            }
        } else if hand[1..=2].iter().all(|&elem| elem == elem2) {
            if hand[3..=4].iter().all(|&elem| elem == elem4) {
                return HandType::TwoPair;
            }
        }

        // Check for a One pair
        // Possibilities: AAxyz, xAAyz, xyAAz, xyzAA
        if hand[0..=1].iter().all(|&elem| elem == elem2)
            || hand[1..=2].iter().all(|&elem| elem == elem2)
            || hand[2..=3].iter().all(|&elem| elem == elem4)
            || hand[3..=4].iter().all(|&elem| elem == elem4)
        {
            return HandType::OnePair;
        }

        return HandType::HighCard;
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
                unreachable!()
            }
        }
    }
}

// The hand types are ordered from the smallest to the biggest
// value as written here.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
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
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => unreachable!(),
        }
    }
}

#[test]
fn card_type_test() {
    assert_eq!(Hand::from("QQQQQ").get_type(), HandType::FiveOfKind);
    assert_eq!(Hand::from("KKAKK").get_type(), HandType::FourOfKind);
    assert_eq!(Hand::from("AAKAA").get_type(), HandType::FourOfKind);
    assert_eq!(Hand::from("KAKAA").get_type(), HandType::FullHouse);
    assert_eq!(Hand::from("AKAKK").get_type(), HandType::FullHouse);
    assert_eq!(Hand::from("AKAA2").get_type(), HandType::ThreeOfKind);
    assert_eq!(Hand::from("2A22K").get_type(), HandType::ThreeOfKind);
    assert_eq!(Hand::from("2KKKA").get_type(), HandType::ThreeOfKind);
    assert_eq!(Hand::from("QJQKJ").get_type(), HandType::TwoPair);
    assert_eq!(Hand::from("JJQKQ").get_type(), HandType::TwoPair);
    assert_eq!(Hand::from("QJQK5").get_type(), HandType::OnePair);
    assert_eq!(Hand::from("TJKJ5").get_type(), HandType::OnePair);
    assert_eq!(Hand::from("527QT").get_type(), HandType::HighCard);
    assert_eq!(Hand::from("5K7A3").get_type(), HandType::HighCard);

    assert!(HandType::FourOfKind < HandType::FiveOfKind);
    assert!(HandType::OnePair > HandType::HighCard);
}
