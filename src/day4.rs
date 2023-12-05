use std::{collections::HashSet, num::ParseIntError, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

struct Scratchcard {
    number: usize,
    winning: HashSet<usize>,
    yours: HashSet<usize>,
}

impl FromStr for Scratchcard {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Card ").unwrap();
        let (card_nr, numbers) = s.split_once(':').unwrap();
        let (winning, yours) = numbers.split_once('|').unwrap();
        Ok(Self {
            number: card_nr.trim().parse::<usize>()? - 1,
            winning: str_numbers_to_set(winning)?,
            yours: str_numbers_to_set(yours)?,
        })
    }
}

fn str_numbers_to_set(s: &str) -> Result<HashSet<usize>, ParseIntError> {
    s.split(' ')
        .filter(|n| !n.is_empty())
        .map(usize::from_str)
        .collect()
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Scratchcard> {
    input
        .lines()
        .map(Scratchcard::from_str)
        .map(Result::unwrap)
        .collect()
}

#[aoc(day4, part1)]
fn part1(cards: &[Scratchcard]) -> usize {
    cards
        .iter()
        .map(|card| card.winning.intersection(&card.yours).count())
        .filter(|n| *n != 0)
        .map(|n| 2usize.pow((n as u32) - 1))
        .sum()
}

#[aoc(day4, part2)]
fn part2(cards: &[Scratchcard]) -> usize {
    let mut total_cards = vec![1; cards.len()];
    for card in cards {
        let winnings = card.winning.intersection(&card.yours).count();
        let count_of_current_card = total_cards[card.number];
        for i in 0..winnings {
            if let Some(card_number) = total_cards.get_mut(card.number + i + 1) {
                *card_number += count_of_current_card;
            }
        }
    }
    total_cards.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static INPUT: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 30);
    }
}
