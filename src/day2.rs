use std::{str::FromStr, collections::{HashMap, hash_map::RandomState}};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Game {
    id: usize,
    games: Vec<HashMap<Color, usize>>
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(format!("Unknown color received: {s}")),
        }
    }
}

fn single_color_draw_to_pair(single_color: &str) -> (Color, usize) {
    let (count, color) = single_color.split_once(' ').unwrap();
    (color.parse().unwrap(), count.parse().unwrap())
}

fn line_to_game(line: &str) -> Game {
    let (game, draws) = line.split_once(": ").unwrap();
    let id: usize = game[5..].parse().unwrap();
    let games = draws
        .split("; ")
        .map(|game| {
            game
                .split(", ")
                .map(single_color_draw_to_pair)
                .collect()
        })
        .collect();
    Game { id, games }
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(line_to_game)
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Game]) -> usize {
    let limits: HashMap<Color, usize, RandomState> = HashMap::from_iter([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);
    input
        .iter()
        .filter(|n| {
            !n.games.iter().flatten().any(|(color, count)| limits[color] < *count)
        })
        .map(|n| n.id)
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Game]) -> usize {
    let mut input = input.to_vec();
    input
        .iter_mut()
        .map(|n| {
            n.games.iter_mut().reduce(|acc, draw| {
                for (color, count) in draw.iter_mut() {
                    acc.entry(*color).and_modify(|n| *n = (*n).max(*count)).or_insert(*count);
                }
                acc
            })
        })
        .map(|game| game.unwrap().values().product::<usize>())
        .sum()
}


#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;
    static INPUT: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 8);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 2286);
    }
}
