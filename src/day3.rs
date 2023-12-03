use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

struct Scemantic {
    numbers: HashMap<Position, Number>,
    symbols: HashMap<Position, char>,
    grid_x: usize,
    grid_y: usize,
}

struct Number {
    value: usize,
    length: usize,
}

impl Number {
    fn new(value: usize, length: usize) -> Self {
        Self{ value, length }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0, value.1)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CharType {
    Void,
    Symbol,
    Number,
}

impl TryFrom<char> for CharType {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            c if c.is_ascii_digit() => Ok(Self::Number),
            '.' => Ok(Self::Void),
            c if c.is_ascii_punctuation() => Ok(Self::Symbol),
            c => Err(format!("Unknown character received: {c}")),
        }
    }
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Scemantic {
    let mut numbers: HashMap<Position, Number> = HashMap::new();
    let mut symbols: HashMap<Position, char> = HashMap::new();
    let mut grid_x: usize = 0;
    let mut grid_y: usize = 0;

    for (y, line) in input.lines().enumerate() {
        grid_y = y + 1;
        let mut value = 0;
        let mut value_length = 0;
        let mut start_possition = 0;
        let mut last_value = false;
        grid_x = line.len();
        for (x, c) in line.char_indices() {
            let ctype: CharType = c.try_into().unwrap();
            if ctype == CharType::Number {
                if !last_value {
                    last_value = true;
                    start_possition = x
                }
                value *= 10;
                value += c.to_digit(10).unwrap() as usize;
                value_length += 1;
            }
            if last_value && (ctype != CharType::Number || x == grid_x - 1) {
                numbers.insert((start_possition, y).into(), Number::new(value, value_length));
                value = 0;
                value_length = 0;
                last_value = false;
            }
            if ctype == CharType::Symbol {
                symbols.insert((x, y).into(), c);
            }
        }
    }
    Scemantic { numbers, symbols, grid_x, grid_y }
}

#[aoc(day3, part1)]
fn part1(scemantic: &Scemantic) -> usize {
    scemantic
        .numbers
        .iter()
        .filter(|(pos, number)| check_near_symbol(pos, number, scemantic))
        .map(|(_pos, number)| number.value)
        .sum()
}

fn check_near_symbol(pos: &Position, number: &Number, scemantics: &Scemantic) -> bool {
    let x = scemantics.grid_x;
    let y = scemantics.grid_y;

    let x_range_start = pos.x.saturating_sub(1);
    let y_range_start = pos.y.saturating_sub(1);

    let x_range_end = x.min(pos.x + number.length + 1);
    let y_range_end = y.min(pos.y + 2);

    (x_range_start..x_range_end)
        .cartesian_product(y_range_start..y_range_end)
        .any(|pos| scemantics.symbols.contains_key(&pos.into()))
}

#[aoc(day3, part2)]
fn part2(scemantic: &Scemantic) -> usize {
    let mut possible_gears: HashMap<Position, Vec<usize>> = HashMap::new();
    for (pos, number) in scemantic.numbers.iter() {
        insert_possible_gears(&mut possible_gears, pos, number, scemantic);
    }
    possible_gears
        .into_values()
        .filter(|v| v.len() == 2)
        .map(|n| n.into_iter().product::<usize>())
        .sum()
}

fn insert_possible_gears(possible_gears: &mut HashMap<Position, Vec<usize>>, pos: &Position, number: &Number, scemantics: &Scemantic) {
    let x = scemantics.grid_x;
    let y = scemantics.grid_y;

    let x_range_start = pos.x.saturating_sub(1);
    let y_range_start = pos.y.saturating_sub(1);

    let x_range_end = x.min(pos.x + number.length + 1);
    let y_range_end = y.min(pos.y + 2);

    for pos in (x_range_start..x_range_end).cartesian_product(y_range_start..y_range_end) {
        let pos: Position = pos.into();
        if scemantics.symbols.get(&pos) == Some(&'*') {
            possible_gears.entry(pos).or_default().push(number.value);
        }
    }

}


#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static INPUT: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};


    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 4361);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 467835);
    }
}
