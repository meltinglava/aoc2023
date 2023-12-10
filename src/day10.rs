use std::{
    collections::HashSet,
    ops::{Index, Not},
};

use aoc_runner_derive::{aoc, aoc_generator};

struct Grid<const N: usize, const M: usize> {
    start: (usize, usize),
    grid: [[Pipe; N]; M],
}

impl<const N: usize, const M: usize> Index<(usize, usize)> for Grid<N, M> {
    type Output = Pipe;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.grid.index(index.1).index(index.0)
    }
}

impl<const N: usize, const M: usize> Grid<N, M> {
    fn step(
        &self,
        from_possition: (usize, usize),
        direction: Direction,
        limit: usize,
    ) -> Option<((usize, usize), Direction)> {
        let new_pos: (usize, usize) = direction.step(from_possition, limit)?;
        if self[new_pos] == Start {
            return Some((new_pos, North));
        }
        let new_direction = self[new_pos].find_exit(direction)?;
        Some((new_pos, new_direction))
    }

    fn find_pipe_start(&self) -> Pipe {
        let mut parts = [(North, M), (South, M), (East, N), (West, N)]
            .into_iter()
            .filter(|(dir, limit)| self.step(self.start, *dir, *limit).is_some())
            .map(|n| n.0);
        let pipe = match parts.next().unwrap() {
            North => match parts.next().unwrap() {
                North => unreachable!("north north no direction"),
                South => NS,
                East => NE,
                West => NW,
            },
            South => match parts.next().unwrap() {
                North | South => unreachable!(),
                East => SE,
                West => SW,
            },
            East => match parts.next().unwrap() {
                North | South | East => unreachable!(),
                West => EW,
            },
            West => unreachable!("found only one direction"),
        };
        debug_assert!(parts.next().is_none());
        pipe
    }

    fn find_steps(&self) -> usize {
        let loop_len = [(North, M), (South, M), (East, N), (West, N)]
            .into_iter()
            .find_map(|(mut dir, limit)| -> Option<usize> {
                let mut pos = self.start;
                let mut n = 0;
                while self[pos] != Start || n == 0 {
                    let next = self.step(pos, dir, limit)?;
                    n += 1;
                    pos = next.0;
                    dir = next.1;
                }
                Some(n)
            })
            .unwrap();
        loop_len / 2
    }

    fn find_loop_coords(&self) -> HashSet<(usize, usize)> {
        [(North, M), (South, M), (East, N), (West, N)]
            .into_iter()
            .find_map(|(mut dir, limit)| -> Option<HashSet<(usize, usize)>> {
                let mut pipe_loop = HashSet::new();
                let mut pos = self.start;
                let mut n = 0;
                while self[pos] != Start || n == 0 {
                    let next = self.step(pos, dir, limit)?;
                    n += 1;
                    pos = next.0;
                    dir = next.1;
                    pipe_loop.insert(pos);
                }
                Some(pipe_loop)
            })
            .unwrap()
    }

    fn find_enclosed_possitions(&self) -> usize {
        let loop_coords = self.find_loop_coords();
        let mut found = 0;
        {
            for x in 0..N {
                let mut inside = false;
                let mut cross_direction = North;
                let mut on_pipe = false;
                for y in 0..M {
                    let pos = (x, y);
                    if loop_coords.contains(&pos) {
                        let mut pos = self[pos];
                        if pos == Start {
                            pos = self.find_pipe_start();
                        }
                        match pos.find_exit(if on_pipe { South } else { North }) {
                            Some(dir) => match dir {
                                North | South => on_pipe = true,
                                West | East => {
                                    if on_pipe {
                                        inside ^= cross_direction != dir;
                                        on_pipe = false;
                                    } else {
                                        cross_direction = dir;
                                        on_pipe = true;
                                    }
                                }
                            },
                            None => {
                                inside ^= true;
                            }
                        }
                    } else {
                        debug_assert!(!on_pipe);
                        if inside {
                            found += 1;
                        }
                    }
                }
            }
        }
        found
    }
}

// N north
// S south
// E east
// W west
// NA = no pipe

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Pipe {
    NS,
    NE,
    NW,
    SE,
    SW,
    EW,
    Ground,
    Start,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Not for Direction {
    type Output = Direction;

    fn not(self) -> Self::Output {
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
}

use Direction::*;
use Pipe::*;

impl TryFrom<char> for Pipe {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(NS),
            '-' => Ok(EW),
            'L' => Ok(NE),
            'J' => Ok(NW),
            '7' => Ok(SW),
            'F' => Ok(SE),
            '.' => Ok(Ground),
            'S' => Ok(Start),
            _ => Err(value),
        }
    }
}

impl Pipe {
    fn find_exit(&self, from_direction: Direction) -> Option<Direction> {
        match from_direction {
            South => match self {
                NS => Some(South),
                NE => Some(East),
                NW => Some(West),
                _ => None,
            },
            North => match self {
                NS => Some(North),
                SE => Some(East),
                SW => Some(West),
                _ => None,
            },
            West => match self {
                NE => Some(North),
                SE => Some(South),
                EW => Some(West),
                _ => None,
            },
            East => match self {
                NW => Some(North),
                SW => Some(South),
                EW => Some(East),
                _ => None,
            },
        }
    }
}

fn checked_add_max(a: usize, b: usize, limit: usize) -> Option<usize> {
    a.checked_add(b).filter(|n| *n < limit)
}

impl Direction {
    fn step(&self, pos: (usize, usize), n: usize) -> Option<(usize, usize)> {
        let x = pos.0;
        let y = pos.1;
        match self {
            North => Some((x, y.checked_sub(1)?)),
            South => Some((x, checked_add_max(y, 1, n)?)),
            East => Some((checked_add_max(x, 1, n)?, y)),
            West => Some((x.checked_sub(1)?, y)),
        }
    }
}

fn parse_grid<const N: usize, const M: usize>(input: &str) -> Grid<N, M> {
    let mut grid = [[Ground; N]; M];
    let mut start = None;
    for (y, line) in input.lines().enumerate() {
        assert_eq!(line.len(), N, "input: {line}");
        for (x, c) in line.char_indices() {
            let point = c.try_into().unwrap();
            grid[y][x] = point;
            if point == Start {
                start = Some((x, y));
            }
        }
    }
    Grid {
        start: start.unwrap(),
        grid,
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Grid<140, 140> {
    parse_grid(input)
}

#[aoc(day10, part1)]
fn part1<const N: usize, const M: usize>(grid: &Grid<N, M>) -> usize {
    grid.find_steps()
}

#[aoc(day10, part2)]
fn part2<const N: usize, const M: usize>(grid: &Grid<N, M>) -> usize {
    grid.find_enclosed_possitions()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static INPUT1_CLEAN: &str = indoc! {"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    "};

    static INPUT1: &str = indoc! {"
        -L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF
    "};

    static INPUT2_CLEAN: &str = indoc! {"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "};

    static INPUT2: &str = indoc! {"
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
    "};

    static PART2_2: &str = indoc! {"
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    "};

    #[test]
    fn test_parse() {
        for input in [INPUT1_CLEAN, INPUT1, INPUT2_CLEAN, INPUT2_CLEAN] {
            parse_grid::<5, 5>(input);
        }
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_grid::<5, 5>(INPUT1)), 4);
        assert_eq!(part1(&parse_grid::<5, 5>(INPUT2)), 8);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_grid::<20, 10>(PART2_2)), 10);
    }
}
