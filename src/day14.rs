use std::{
    assert_matches::assert_matches,
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        BTreeMap, HashMap,
    },
    fmt::{Debug, Display},
};

use aoc_runner_derive::{aoc, aoc_generator};

use indexmap::IndexMap;
use itertools::Itertools;
use Direction::*;
use RockType::*;

type GridMap = BTreeMap<(usize, usize), RockType>;

#[derive(PartialEq, Eq, Hash, Clone)]
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
        writeln!(f, "\n{}", grid)
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum RockType {
    CubeRock,
    RoundRock,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
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

impl Direction {
    fn cross_direction_iter(&self, grid: &Grid) -> impl Iterator<Item = usize> {
        match self {
            North | South => 0..grid.size.0,
            East | West => 0..grid.size.1,
        }
    }

    fn with_direction_iter(&self, grid: &Grid) -> Box<dyn Iterator<Item = usize>> {
        match self {
            North => Box::new(0..grid.size.1),
            South => Box::new((0..grid.size.1).rev()),
            East => Box::new((0..grid.size.0).rev()),
            West => Box::new(0..grid.size.0),
        }
    }

    fn with_direction_start(&self, grid: &Grid) -> usize {
        match self {
            North => 0,
            South => grid.size.1 - 1,
            East => grid.size.0 - 1,
            West => 0,
        }
    }

    fn cross_with_to_coord(&self, cross_direction: usize, with_direction: usize) -> (usize, usize) {
        match self {
            North | South => (cross_direction, with_direction),
            East | West => (with_direction, cross_direction),
        }
    }

    fn with_direction_to_next_avalible(&self, with_direction: usize) -> usize {
        match self {
            North | West => with_direction + 1,
            South | East => with_direction.saturating_sub(1),
        }
    }
}

impl Grid {
    fn roll_direction(&self, direction: Direction) -> Self {
        let mut rolled = GridMap::new();
        for cross_direction in direction.cross_direction_iter(self) {
            let mut next_avalible = direction.with_direction_start(self);
            for with_direction in direction.with_direction_iter(self) {
                let coord = direction.cross_with_to_coord(cross_direction, with_direction);
                if let Some(rock) = self.grid.get(&coord) {
                    match rock {
                        CubeRock => {
                            assert_matches!(rolled.insert(coord, CubeRock), None);
                            next_avalible =
                                direction.with_direction_to_next_avalible(with_direction);
                        }
                        RoundRock => {
                            assert_matches!(
                                rolled.insert(
                                    direction.cross_with_to_coord(cross_direction, next_avalible),
                                    RoundRock
                                ),
                                None,
                                "{:?}, {:?}",
                                (cross_direction, next_avalible),
                                direction
                            );
                            next_avalible =
                                direction.with_direction_to_next_avalible(next_avalible);
                        }
                    }
                }
            }
        }
        Self {
            grid: rolled,
            size: self.size,
        }
    }

    fn total_load(&self) -> usize {
        self.grid
            .iter()
            .filter(|&(_, rock)| *rock == RoundRock)
            .map(|(pos, _)| self.size.1 - pos.1)
            .sum()
    }

    fn cycle(self) -> Self {
        let mut last = self;
        for dir in [North, West, South, East] {
            last = last.roll_direction(dir);
        }
        last
    }

    fn total_load_shake(&self, cycles: usize) -> usize {
        let mut cache: HashMap<Grid, usize> = HashMap::new();
        let mut lookup: IndexMap<usize, Grid> = IndexMap::new();
        let mut current: Self = self.clone();
        let mut cycle = 0;
        let mut found = 0;
        for _ in 0..cycles {
            match cache.entry(current.clone()) {
                Vacant(vacant) => {
                    lookup.insert(cycle, vacant.key().clone());
                    vacant.insert(cycle);
                }
                Occupied(occupied) => {
                    found = *occupied.get();
                    break;
                }
            }
            current = current.cycle();
            cycle += 1;
        }
        dbg!(&lookup, found);
        let offset = (cycles - found) % (cycle - found) + found;
        lookup.get(&offset).unwrap().total_load()
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
    grid.roll_direction(North).total_load()
}

#[aoc(day14, part2)]
fn part2(grid: &Grid) -> usize {
    grid.total_load_shake(1000000000)
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
        assert_eq!(&parse(INPUT).roll_direction(North), &parse(ROLLED))
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 136);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 64);
    }
}
