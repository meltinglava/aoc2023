use std::{cmp::Ordering, num::ParseIntError, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug)]
struct Hand {
    cards: [char; 5],
    bid: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

use HandType::*;

impl HandType {
    fn from_hand_p1(hand: &Hand) -> Self {
        let cards = hand.cards.iter().copied().counts();
        let mut selsections = cards.values().sorted().rev();
        match selsections.next().unwrap() {
            5 => FiveOfAKind,
            4 => FourOfAKind,
            3 => match selsections.next().unwrap() {
                2 => FullHouse,
                1 => ThreeOfAKind,
                _ => unreachable!("More than 5 cards in one hand"),
            },
            2 => match selsections.next().unwrap() {
                2 => TwoPair,
                1 => OnePair,
                _ => unreachable!("More than 5 cards in one hand"),
            },
            1 => HighCard,
            _ => unreachable!("More than 5 cards in one hand"),
        }
    }

    fn from_hand_p2(hand: &Hand) -> Self {
        let cards = hand.cards.iter().copied().counts();
        let mut selsections = cards.into_iter().sorted_by_key(|n| n.1).rev();
        match selsections.next().unwrap() {
            (_, 5) => FiveOfAKind,
            (v0, 4) => {
                if v0 == 'J' || selsections.next().unwrap().0 == 'J' {
                    FiveOfAKind
                } else {
                    FourOfAKind
                }
            }
            (v0, 3) => match selsections.next().unwrap() {
                (v1, 2) => {
                    if v0 == 'J' || v1 == 'J' {
                        FiveOfAKind
                    } else {
                        FullHouse
                    }
                }
                (v1, 1) => {
                    if v0 == 'J' || v1 == 'J' || selsections.next().unwrap().0 == 'J' {
                        FourOfAKind
                    } else {
                        ThreeOfAKind
                    }
                }
                (_v, _) => unreachable!("More than 5 cards in one hand"),
            },
            (v0, 2) => match selsections.next().unwrap() {
                (v1, 2) => match selsections.next().unwrap().0 {
                    'J' => FullHouse,
                    _ => {
                        if v0 == 'J' || v1 == 'J' {
                            FourOfAKind
                        } else {
                            TwoPair
                        }
                    }
                },
                (v1, 1) => {
                    if v0 == 'J' || v1 == 'J' || selsections.any(|n| n.0 == 'J') {
                        ThreeOfAKind
                    } else {
                        OnePair
                    }
                }
                (_v, _) => unreachable!("More than 5 cards in one hand"),
            },
            (v0, 1) => {
                if v0 == 'J' || selsections.any(|n| n.0 == 'J') {
                    OnePair
                } else {
                    HighCard
                }
            }
            (_v, _) => unreachable!("More than 5 cards in one hand"),
        }
    }
}

impl FromStr for Hand {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (chars, bid) = s.trim().split_once(' ').unwrap();
        let mut cards = [' '; 5];
        chars.char_indices().for_each(|(n, c)| cards[n] = c);
        Ok(Self {
            cards,
            bid: bid.parse()?,
        })
    }
}

const ORDER_P1: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];
const ORDER_P2: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

fn char_to_value_p1(c: &char) -> usize {
    13 - ORDER_P1.iter().find_position(|&n| n == c).unwrap().0
}

fn char_to_value_p2(c: &char) -> usize {
    13 - ORDER_P2.iter().find_position(|&n| n == c).unwrap().0
}

fn sort_hands_by_p1(l: &&Hand, r: &&Hand) -> Ordering {
    HandType::from_hand_p1(l)
        .cmp(&HandType::from_hand_p1(r))
        .then_with(|| {
            l.cards
                .iter()
                .map(char_to_value_p1)
                .cmp(r.cards.iter().map(char_to_value_p1))
        })
}

fn sort_hands_by_p2(l: &&Hand, r: &&Hand) -> Ordering {
    HandType::from_hand_p2(l)
        .cmp(&HandType::from_hand_p2(r))
        .then_with(|| {
            l.cards
                .iter()
                .map(char_to_value_p2)
                .cmp(r.cards.iter().map(char_to_value_p2))
        })
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Hand> {
    input.lines().map(Hand::from_str).try_collect().unwrap()
}

#[aoc(day7, part1)]
fn part1(hands: &[Hand]) -> usize {
    hands
        .iter()
        .sorted_by(sort_hands_by_p1)
        .enumerate()
        .map(|(n, hand)| (n + 1) * hand.bid)
        .sum()
}

#[aoc(day7, part2)]
fn part2(hands: &[Hand]) -> usize {
    hands
        .iter()
        .sorted_by(sort_hands_by_p2)
        .enumerate()
        .map(|(n, hand)| (n + 1) * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static INPUT: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 6440);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 5905);
    }
}
