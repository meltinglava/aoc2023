use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
    str::FromStr,
};

use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;
use Direction::*;
use Tile::*;

type Seen = HashMap<(usize, usize), HashSet<Direction>>;

struct Grid<const N: usize> {
    grid: [[Tile; N]; N],
}

impl<const N: usize> Display for Grid<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid = self
            .grid
            .iter()
            .map(|line| {
                line.iter()
                    .map(|tile| char::from(*tile))
                    .collect::<String>()
            })
            .join("\n");
        writeln!(f, "\n{}", grid)
    }
}

impl<const N: usize> Debug for Grid<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<const N: usize> Index<(usize, usize)> for Grid<N> {
    type Output = Tile;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.grid.index(index.1).index(index.0)
    }
}

impl<const N: usize> IndexMut<(usize, usize)> for Grid<N> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.grid.index_mut(index.1).index_mut(index.0)
    }
}

impl<const N: usize> FromStr for Grid<N> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Self {
            grid: [[Empty; N]; N],
        };
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.char_indices() {
                grid[(x, y)] = Tile::try_from(c).unwrap();
            }
        }
        Ok(grid)
    }
}

impl<const N: usize> Grid<N> {
    #[allow(unused)]
    fn display_lighted_up(&self, seen: &Seen) -> String {
        (0..self.grid.len())
            .map(|y| {
                (0..self.grid[0].len())
                    .map(|x| match seen.get(&(x, y)) {
                        Some(_) => '#',
                        None => '.',
                    })
                    .collect::<String>()
            })
            .join("\n")
    }

    fn light_up(&self) -> usize {
        let mut seen = HashMap::new();
        self.recurse_light_up((0, 0), East, &mut seen);
        seen.len()
    }

    fn light_up_all(&self) -> usize {
        [North, East, West, South]
            .into_iter()
            .flat_map(|dir| dir.edge_iterator::<N>())
            .map(|(pos, dir)| {
                let mut seen = HashMap::new();
                self.recurse_light_up(pos, dir, &mut seen);
                seen.len()
            })
            .max()
            .unwrap()
    }

    fn recurse_light_up(
        &self,
        possition: (usize, usize),
        mut direction: Direction,
        seen: &mut Seen,
    ) {
        if !seen.entry(possition).or_default().insert(direction) {
            return;
        }
        match self[possition] {
            Empty => (),
            MirrorNorthEast => {
                direction = match direction {
                    North => East,
                    South => West,
                    East => North,
                    West => South,
                };
            }
            MirrorNorthWest => {
                direction = match direction {
                    North => West,
                    South => East,
                    East => South,
                    West => North,
                };
            }
            SplitterHorisontal => {
                direction = match direction {
                    North | South => {
                        if let Some(step) = West.step::<N>(possition) {
                            self.recurse_light_up(step, West, seen);
                        }
                        East
                    }
                    East | West => direction,
                };
            }
            SplitterVertical => {
                direction = match direction {
                    East | West => {
                        if let Some(step) = North.step::<N>(possition) {
                            self.recurse_light_up(step, North, seen);
                        }
                        South
                    }
                    North | South => direction,
                };
            }
        }
        if let Some(step) = direction.step::<N>(possition) {
            self.recurse_light_up(step, direction, seen);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Empty,
    MirrorNorthEast,
    MirrorNorthWest,
    SplitterHorisontal,
    SplitterVertical,
}

impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Empty),
            '/' => Ok(MirrorNorthEast),
            '\\' => Ok(MirrorNorthWest),
            '-' => Ok(SplitterHorisontal),
            '|' => Ok(SplitterVertical),
            c => Err(c),
        }
    }
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Empty => '.',
            MirrorNorthEast => '/',
            MirrorNorthWest => '\\',
            SplitterHorisontal => '-',
            SplitterVertical => '|',
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn step<const N: usize>(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            North => Some((pos.0, pos.1.checked_sub(1)?)),
            South => {
                let added = pos.1 + 1;
                if added < N {
                    Some((pos.0, added))
                } else {
                    None
                }
            }
            East => {
                let added = pos.0 + 1;
                if added < N {
                    Some((added, pos.1))
                } else {
                    None
                }
            }
            West => Some((pos.0.checked_sub(1)?, pos.1)),
        }
    }

    fn edge_iterator<const N: usize>(&self) -> Box<dyn Iterator<Item = ((usize, usize), Self)>> {
        match self {
            North => Box::new((0..N).map(|n| ((n, N - 1), North))),
            South => Box::new((0..N).map(|n| ((n, 0), South))),
            East => Box::new((0..N).map(|n| ((0, n), East))),
            West => Box::new((0..N).map(|n| ((N - 1, n), West))),
        }
    }
}

fn parse_grid<const N: usize>(input: &str) -> Grid<N> {
    input.parse().unwrap()
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Grid<110> {
    parse_grid(input)
}

#[aoc(day16, part1)]
fn part1<const N: usize>(grid: &Grid<N>) -> usize {
    grid.light_up()
}

#[aoc(day16, part2)]
fn part2<const N: usize>(grid: &Grid<N>) -> usize {
    grid.light_up_all()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static INPUT: &str = indoc! {r"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_grid::<10>(INPUT)), 46);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_grid::<10>(INPUT)), 51);
    }
}
