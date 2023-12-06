use std::{num::ParseIntError, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl From<(usize, usize)> for Race {
    fn from(value: (usize, usize)) -> Self {
        Self {
            time: value.0,
            distance: value.1,
        }
    }
}

fn parse_line_numbers(input: &str) -> Vec<usize> {
    str_numbers_to_collecatble(&input[9..]).unwrap()
}

fn parse_line_bad_kerning(input: &str) -> usize {
    input[9..].trim().replace(' ', "").parse().unwrap()
}

fn winning_times(race: &Race) -> usize {
    // use the qadratic equation to solve this
    // a = -1
    // b = race.time (t)
    // c = -race.distance (d)
    let t = race.time as f64;
    // we need beat the record, not tangent it
    // 0.1 is ok as distances we are looking for will always be whole numbers
    let d = race.distance as f64 + 0.1;
    let root = (t.powi(2) - 4. * d).sqrt();
    let a = (((-t - root) / -2.).floor()) as usize;
    let b = (((-t + root) / -2.).ceil()) as usize;
    a - b + 1
}

#[aoc_generator(day6, part1)]
fn parse_part1(input: &str) -> Vec<Race> {
    let (time_str, distance_str) = input.split_once('\n').unwrap();
    parse_line_numbers(time_str)
        .into_iter()
        .zip(parse_line_numbers(distance_str))
        .map(Race::from)
        .collect()
}

#[aoc_generator(day6, part2)]
fn parse_part2(input: &str) -> Race {
    let (time_str, distance_str) = input.split_once('\n').unwrap();
    Race {
        time: parse_line_bad_kerning(time_str),
        distance: parse_line_bad_kerning(distance_str),
    }
}

#[aoc(day6, part1)]
fn part1(input: &[Race]) -> usize {
    input.iter().map(winning_times).product()
}

#[aoc(day6, part2)]
fn part2(race: &Race) -> usize {
    winning_times(race)
}

fn str_numbers_to_collecatble<T>(s: &str) -> Result<T, ParseIntError>
where
    T: FromIterator<usize>,
{
    s.trim()
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(str::trim)
        .map(usize::from_str)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static INPUT: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_part1(INPUT)), 288);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_part2(INPUT)), 71503);
    }
}
