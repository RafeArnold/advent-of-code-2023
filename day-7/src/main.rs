use std::cmp::Ordering;
use std::num::ParseIntError;
use std::ops::Sub;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
    println!("{}", run_2(INPUT));
}

fn run_1(input: &str) -> usize {
    run::<Hand, _>(input, parse_line)
}

fn run_2(input: &str) -> usize {
    run::<Hand2, _>(input, parse_line_2)
}

fn run<H, F>(input: &str, parse_line: F) -> usize
where
    H: Ord,
    F: Fn(&str) -> (H, usize),
{
    let mut hands = input.lines().map(parse_line).collect::<Vec<_>>();
    hands.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, (_hand, bid))| (rank + 1) * bid)
        .sum()
}

fn parse_line_2(line: &str) -> (Hand2, usize) {
    let (hand, bid) = parse_line(line);
    (hand.into(), bid)
}

fn parse_line(line: &str) -> (Hand, usize) {
    let (cards, bid) = line.split_once(' ').unwrap();
    let hand = Hand::try_from(cards).unwrap();
    let bid = bid.parse().unwrap();
    (hand, bid)
}

#[derive(Debug, Eq)]
struct Hand {
    cards: [Card; 5],
}

#[derive(Debug, Eq, PartialEq)]
struct Hand2 {
    cards: [Card; 5],
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Card {
    Num(u8),
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Card2 {
    J,
    Num(u8),
    T,
    Q,
    K,
    A,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Kind {
    High,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

impl From<&Card> for Card2 {
    fn from(value: &Card) -> Self {
        match value {
            Card::Num(num) => Card2::Num(*num),
            Card::T => Card2::T,
            Card::J => Card2::J,
            Card::Q => Card2::Q,
            Card::K => Card2::K,
            Card::A => Card2::A,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind().cmp(&other.kind()) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            cmp => cmp,
        }
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards.eq(&other.cards)
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind().cmp(&other.kind()) {
            Ordering::Equal => self
                .cards
                .iter()
                .map(Card2::from)
                .cmp(other.cards.iter().map(Card2::from)),
            cmp => cmp,
        }
    }
}

impl PartialOrd<Self> for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn kind(&self) -> Kind {
        let mut groups: [u8; 13] = [0; 13];
        for card in &self.cards {
            groups[card.value() as usize] += 1;
        }
        groups.sort_unstable();
        groups.reverse();
        kind(groups)
    }
}

impl Hand2 {
    fn kind(&self) -> Kind {
        let mut groups: [u8; 13] = [0; 13];
        let mut j_count = 0;
        for card in &self.cards {
            if let Card::J = card {
                j_count += 1;
            } else {
                groups[card.value() as usize] += 1;
            }
        }
        groups.sort_unstable();
        groups.reverse();
        groups[0] += j_count;
        kind(groups)
    }
}

fn kind(groups: [u8; 13]) -> Kind {
    if groups[0] == 5 {
        Kind::Five
    } else if groups[0] == 4 {
        Kind::Four
    } else if groups[0] == 3 && groups[1] == 2 {
        Kind::FullHouse
    } else if groups[0] == 3 {
        Kind::Three
    } else if groups[0] == 2 && groups[1] == 2 {
        Kind::TwoPair
    } else if groups[0] == 2 {
        Kind::Pair
    } else {
        Kind::High
    }
}

impl Card {
    fn value(&self) -> u8 {
        match self {
            Card::Num(num) => *num,
            Card::T => 10,
            Card::J => 11,
            Card::Q => 12,
            Card::K => 13,
            Card::A => 14,
        }
        .sub(2)
    }
}

impl TryFrom<&str> for Hand {
    type Error = ParseHandError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut cards = value.bytes().map(Card::try_from);
        let cards = [
            cards.next().ok_or(ParseHandError::TooFewCards)??,
            cards.next().ok_or(ParseHandError::TooFewCards)??,
            cards.next().ok_or(ParseHandError::TooFewCards)??,
            cards.next().ok_or(ParseHandError::TooFewCards)??,
            cards.next().ok_or(ParseHandError::TooFewCards)??,
        ];
        Ok(Hand { cards })
    }
}

impl From<Hand> for Hand2 {
    fn from(value: Hand) -> Self {
        Hand2 { cards: value.cards }
    }
}

impl TryFrom<u8> for Card {
    type Error = ParseHandError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'0'..=b'9' => Card::Num(value - b'0'),
            b'A' => Card::A,
            b'K' => Card::K,
            b'Q' => Card::Q,
            b'J' => Card::J,
            b'T' => Card::T,
            _ => return Err(ParseHandError::UnrecognisedCard(value)),
        })
    }
}

#[derive(Debug)]
enum ParseHandError {
    ParseInt(ParseIntError),
    UnrecognisedCard(u8),
    TooFewCards,
}

impl From<ParseIntError> for ParseHandError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

impl From<u8> for ParseHandError {
    fn from(value: u8) -> Self {
        Self::UnrecognisedCard(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT), 6440);
    }

    #[test]
    fn challenge_2() {
        assert_eq!(run_2(INPUT), 5905);
    }

    #[test]
    fn cmp() {
        let a = Hand::try_from("KK677").unwrap();
        let b = Hand::try_from("KTJJT").unwrap();
        assert!(a > b);
    }

    #[test]
    fn kind() {
        assert_eq!(Hand::try_from("AAAAA").unwrap().kind(), Kind::Five);
        assert_eq!(Hand::try_from("AA8AA").unwrap().kind(), Kind::Four);
        assert_eq!(Hand::try_from("23332").unwrap().kind(), Kind::FullHouse);
        assert_eq!(Hand::try_from("TTT98").unwrap().kind(), Kind::Three);
        assert_eq!(Hand::try_from("23432").unwrap().kind(), Kind::TwoPair);
        assert_eq!(Hand::try_from("A23A4").unwrap().kind(), Kind::Pair);
        assert_eq!(Hand::try_from("23456").unwrap().kind(), Kind::High);
    }

    #[test]
    fn cmp_2() {
        let a = Hand2::from(Hand::try_from("TTTT2").unwrap());
        let b = Hand2::from(Hand::try_from("JKKK2").unwrap());
        assert_eq!(a.cmp(&b), Ordering::Greater);
        assert!(a > b);
    }

    #[test]
    fn kind_2() {
        let hand = Hand2::from(Hand::try_from("QJJQ2").unwrap());
        assert_eq!(hand.kind(), Kind::Four);
    }
}
