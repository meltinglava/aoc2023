use std::{
    collections::HashMap,
    iter::repeat_with,
    num::ParseIntError,
    str::{pattern::Pattern, FromStr},
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use Field::*;

#[derive(Debug, PartialEq, Eq, Hash)]
struct SpringRow {
    row: Vec<Field>,
    numbers: Vec<usize>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Field {
    Operational,
    Damaged,
    Unknown,
}

impl FromStr for SpringRow {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s_row, numbers) = s.split_once(' ').unwrap();
        let row = s_row
            .chars()
            .map(|c| match c {
                '#' => Damaged,
                '.' => Operational,
                '?' => Unknown,
                _ => unreachable!("Character is unknown: {}", c),
            })
            .collect();
        let numbers = str_numbers_to_collecatble(numbers, ',')?;
        Ok(Self { row, numbers })
    }
}

impl SpringRow {
    fn unfold(&mut self) {
        self.row = repeat_with(|| self.row.clone())
            .take(5)
            .update(|v| v.push(Unknown))
            .flatten()
            .collect();
        self.row.pop();
        self.numbers = repeat_with(|| self.numbers.clone())
            .take(5)
            .flatten()
            .collect();
    }
}

fn count(
    cfg: &[Field],
    nums: &[usize],
    map: &mut HashMap<(Vec<Field>, Vec<usize>), usize>,
) -> usize {
    if cfg.is_empty() {
        return usize::from(nums.is_empty());
    } else if nums.is_empty() {
        return usize::from(!cfg.contains(&Damaged));
    }
    let key = (cfg.to_vec(), nums.to_vec());
    if let Some(value) = map.get(&key) {
        return *value;
    }

    let mut result = 0;
    if cfg[0] != Damaged {
        result += count(&cfg[1..], nums, map);
    }
    if cfg[0] != Operational && nums[0] <= cfg.len() && !cfg[..nums[0]].contains(&Operational) {
        if nums[0] == cfg.len() {
            result += count(&[], &nums[1..], map);
        } else if cfg[nums[0]] != Damaged {
            result += count(&cfg[(nums[0] + 1)..], &nums[1..], map);
        }
    }
    map.insert(key, result);
    result
}

fn str_numbers_to_collecatble<'a, T, N, P>(s: &'a str, p: P) -> Result<T, ParseIntError>
where
    T: FromIterator<N>,
    N: FromStr<Err = ParseIntError>,
    P: Pattern<'a>,
{
    s.split(p)
        .filter(|n| !n.is_empty())
        .map(N::from_str)
        .collect()
}

#[aoc_generator(day12, part1)]
fn parse1(input: &str) -> Vec<SpringRow> {
    input
        .lines()
        .map(SpringRow::from_str)
        .try_collect()
        .unwrap()
}

#[aoc_generator(day12, part2)]
fn parse2(input: &str) -> Vec<SpringRow> {
    let mut map: Vec<_> = input
        .lines()
        .map(SpringRow::from_str)
        .try_collect()
        .unwrap();
    map.iter_mut().for_each(SpringRow::unfold);
    map
}

#[aoc(day12, part1)]
fn part1(rows: &[SpringRow]) -> usize {
    rows.iter()
        .map(|c| count(&c.row, &c.numbers, &mut HashMap::new()))
        .sum()
}

#[aoc(day12, part2)]
fn part2(rows: &[SpringRow]) -> usize {
    rows.iter()
        .map(|c| count(&c.row, &c.numbers, &mut HashMap::new()))
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static INPUT: &str = indoc! {"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    "};

    const INPUT_COMBINATIONS: [usize; 6] = [1, 4, 1, 1, 4, 10];

    #[test]
    fn tests_combinations() {
        for (input, combinations) in INPUT.lines().zip(INPUT_COMBINATIONS) {
            let parsed: SpringRow = input.parse().unwrap();
            assert_eq!(
                count(&parsed.row, &parsed.numbers, &mut HashMap::new()),
                combinations
            );
        }
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse1(INPUT)), 21);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse2(INPUT)), 525152);
    }
}
