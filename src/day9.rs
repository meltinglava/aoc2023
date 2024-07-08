use std::{num::ParseIntError, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

fn str_numbers_to_collecatble<T, N>(s: &str) -> Result<T, ParseIntError>
where
    T: FromIterator<N>,
    N: FromStr<Err = ParseIntError>,
{
    s.split(' ')
        .filter(|n| !n.is_empty())
        .map(N::from_str)
        .collect()
}

fn find_next_number(numbers: &[i64]) -> i64 {
    let mut numbers = vec![numbers.to_vec()];
    while numbers.last().unwrap().iter().copied().any(|n| n != 0) {
        let next = numbers
            .last()
            .unwrap()
            .windows(2)
            .map(|n| n[1] - n[0])
            .collect_vec();
        numbers.push(next);
    }
    let mut last = 0;
    for line in numbers.iter_mut().rev() {
        let line_last = *line.last().unwrap();
        line.push(last + line_last);
        last += line_last;
    }
    last
}

fn find_former_number(numbers: &[i64]) -> i64 {
    let mut numbers = vec![numbers.to_vec()];
    while numbers.last().unwrap().iter().copied().any(|n| n != 0) {
        let next = numbers
            .last()
            .unwrap()
            .windows(2)
            .map(|n| n[1] - n[0])
            .collect_vec();
        numbers.push(next);
    }
    let mut former_first = 0;
    for line in numbers.iter_mut().rev() {
        let line_first = *line.first().unwrap();
        former_first = line_first - former_first;
        line.insert(0, former_first);
    }
    former_first
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Result<Vec<Vec<i64>>, ParseIntError> {
    input.lines().map(str_numbers_to_collecatble).collect()
}

#[aoc(day9, part1, first)]
fn part1(numbers: &[Vec<i64>]) -> i64 {
    numbers.iter().map(|n| find_next_number(n.as_slice())).sum()
}

#[aoc(day9, part2, first)]
fn part2(numbers: &[Vec<i64>]) -> i64 {
    numbers
        .iter()
        .map(|n| find_former_number(n.as_slice()))
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static INPUT: &str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT).unwrap()), 114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT).unwrap()), 2);
    }
}
