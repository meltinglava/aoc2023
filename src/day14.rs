use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;
use RockType::*;

type GridMap = HashMap<(usize, usize), RockType>;

#[derive(PartialEq, Eq)]
struct Grid {
    grid: GridMap,
    size: (usize, usize),
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid = (0..self.size.1)
            .map(|y| {
                (0..self.size.0)
                    .map(|x| {
                        self.grid
                            .get(&(x, y))
                            .copied()
                            .map(char::from)
                            .unwrap_or('.')
                    })
                    .collect::<String>()
            })
            .join("\n");
        writeln!(f, "{}", grid)
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum RockType {
    CubeRock,
    RoundRock,
}

impl TryFrom<char> for RockType {
    type Error = char;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'O' => Ok(RoundRock),
            '#' => Ok(CubeRock),
            '.' => Err(c),
            _ => unreachable!("Unknown rock: {}", c),
        }
    }
}

impl From<RockType> for char {
    fn from(value: RockType) -> Self {
        match value {
            CubeRock => '#',
            RoundRock => 'O',
        }
    }
}

impl Grid {
    fn roll(&self) -> Self {
        let mut rolled = GridMap::with_capacity(self.grid.len());
        for x in 0..self.size.0 {
            let mut next_avalible = 0;
            for y in 0..self.size.1 {
                if let Some(rock) = self.grid.get(&(x, y)) {
                    match rock {
                        CubeRock => {
                            rolled.insert((x, y), CubeRock);
                            next_avalible = y + 1;
                        }
                        RoundRock => {
                            rolled.insert((x, next_avalible), RoundRock);
                            next_avalible += 1;
                        }
                    }
                }
            }
        }
        Self {
            grid: rolled,
            size: self.size.clone(),
        }
    }

    fn total_load(&self) -> usize {
        self.grid
            .iter()
            .filter(|&(_, rock)| *rock == RoundRock)
            .map(|(pos, _)| self.size.1 - pos.1)
            .sum()
    }
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Grid {
    let mut grid = GridMap::new();
    let mut x_max = 0;
    let mut y_max = 0;
    for (y, line) in input.lines().enumerate() {
        y_max = y_max.max(y + 1);
        for (x, c) in line.char_indices() {
            x_max = x_max.max(x + 1);
            if let Ok(rock) = RockType::try_from(c) {
                grid.insert((x, y), rock);
            }
        }
    }
    Grid {
        grid,
        size: (x_max, y_max),
    }
}

#[aoc(day14, part1)]
fn part1(grid: &Grid) -> usize {
    grid.roll_north().total_load()
}

#[aoc(day14, part2)]
fn part2(grid: &Grid) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static INPUT: &str = indoc! {"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "};

    static ROLLED: &str = indoc! {"
        OOOO.#.O..
        OO..#....#
        OO..O##..O
        O..#.OO...
        ........#.
        ..#....#.#
        ..O..#.O.O
        ..O.......
        #....###..
        #....#....
    "};

    #[test]
    fn test_rolled() {
        assert_eq!(&parse(INPUT).roll_north(), &parse(ROLLED))
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 136);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 0);
    }
}
