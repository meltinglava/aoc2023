use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}, fmt::{Debug, Display}, ops::{Index, IndexMut, Not, RangeBounds}};
use itertools::Itertools;

use aoc_runner_derive::{aoc, aoc_generator};

type Coord = (usize, usize);

#[derive(Clone)]
struct Grid<const N: usize> {
    grid: [[usize; N]; N],
}

impl<const N: usize> Grid<N> {
    fn end(&self) -> Coord {
        (N - 1, N - 1)
    }
}

impl<const N: usize> Index<Coord> for Grid<N> {
    type Output = usize;

    fn index(&self, index: Coord) -> &Self::Output {
        self.grid.index(index.1).index(index.0)
    }
}

impl<const N: usize> IndexMut<Coord> for Grid<N> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        self.grid.index_mut(index.1).index_mut(index.0)
    }
}

impl<const N: usize> Display for Grid<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.grid.iter().map(|line| line.iter().copied().join("")).join("\n"))
    }
}

impl<const N: usize> Debug for Grid<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn checked_add_to_max(a: usize, b: usize, limit: usize) -> Option<usize> {
    let ans = a + b;
    (..limit).contains(&ans).then_some(ans)
}

impl Direction {
    fn step(&self, coord: Coord, limit: usize) -> Option<Coord> {
        Some(match self {
            North => (coord.0, coord.1.checked_sub(1)?),
            South => (coord.0, checked_add_to_max(coord.1, 1, limit)?),
            East => (checked_add_to_max(coord.0, 1, limit)?, coord.1),
            West => (coord.0.checked_sub(1)?, coord.1),
        })
    }
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

fn coords_iterator(n: usize) -> impl Iterator<Item=Coord> {
    (0..n).flat_map(move |i| (0..n).map(move |j| (j, i)))
}

fn parse_grid<const N: usize>(input: &str) -> Grid<N> {
    let mut grid = Grid::<N> {
        grid: [[usize::MAX; N]; N],
    };
    for (number, coord) in input
        .lines()
        .flat_map(str::chars)
        .map(|c| c.to_digit(10).unwrap() as usize)
        .zip(coords_iterator(N))
    {
        grid[coord] = number
    }
    grid
}

fn possible_moves_by_heat_loss(coord: Coord, direction: Direction, steps_in_direction: usize, limit: usize) -> Vec<(Coord, Direction)> {
    [North, South, East, West]
        .into_iter()
        .filter(move |d| *d != !direction)
        .filter(move |d| *d != direction || steps_in_direction < 3)
        .filter_map(move |d| Some((d.step(coord, limit)?, d)))
        .collect()
}

fn possible_moves_by_ultra_cruciblescrucibles(coord: Coord, direction: Direction, steps_in_direction: usize, limit: usize) -> Vec<(Coord, Direction)> {
    [North, South, East, West]
        .into_iter()
        .filter(move |d| *d != !direction)
        .filter(move |d| (*d == direction && steps_in_direction < 10) || (*d != direction && (4..=10).contains(&steps_in_direction)))
        .filter_map(move |d| Some((d.step(coord, limit)?, d)))
        .collect()
}

impl QueueOrder {
    fn new(cost: usize, pos: Coord, direction: Direction, num_steps: usize) -> Self {
        Self{
            cost,
            pos,
            direction,
            num_steps,
        }
    }

}

fn find_shortest_path<const N: usize, F: Fn(Coord, Direction, usize, usize) -> Vec<(Coord, Direction)>, R: RangeBounds<usize>>(grid: &Grid<N>, moves: F, steps_bounds: R) -> usize {
    let mut seen = HashSet::<(Coord, Direction, usize)>::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(QueueOrder::new(0, (0, 0), South, 0)));
    queue.push(Reverse(QueueOrder::new(0, (0, 0), East, 0)));
    while let Some(tgt) = queue.pop() {
        let tgt = tgt.0;
        assert!(tgt.num_steps < 11);
        if tgt.pos == grid.end() && steps_bounds.contains(&tgt.num_steps) {
            return tgt.cost;
        }
        if !seen.insert((tgt.pos, tgt.direction, tgt.num_steps)) {
            continue;
        }
        let m = moves(tgt.pos, tgt.direction, tgt.num_steps, N);
        for (new_pos, new_direction) in m {
            queue.push(Reverse(QueueOrder::new(tgt.cost + grid[new_pos], new_pos, new_direction, if tgt.direction == new_direction {tgt.num_steps + 1} else {1})));
        }
    }
    unreachable!()
}

#[aoc_generator(day17)]
fn parse(input: &str) -> Grid<141> {
    parse_grid(input)
}

#[aoc(day17, part1)]
fn part1<const N: usize>(grid: &Grid<N>) -> usize {
    find_shortest_path(grid, possible_moves_by_heat_loss, 1..=3)
}

#[aoc(day17, part2)]
fn part2<const N: usize>(grid: &Grid<N>) -> usize {
    find_shortest_path(grid, possible_moves_by_ultra_cruciblescrucibles, 4..=10)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct QueueOrder {
    cost: usize,
    pos: Coord,
    direction: Direction,
    num_steps: usize,
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static INPUT: &str = indoc! {r"
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
    "};

    #[test]
    fn test_parse() {
        assert_eq!(INPUT.trim_end(), parse_grid::<13>(INPUT).to_string())
    }

    #[test]
    fn test_step() {
        let grid: Grid<13> = parse_grid::<13>(INPUT);
        let step = East.step((0, 0), 13).unwrap();
        assert_eq!(step, (1, 0));
        assert_eq!(grid[step], 4)
    }

    #[test]
    fn test_parse_full_input() {
        let input = std::fs::read_to_string("input/2023/day17.txt").unwrap();
        assert_eq!(input.trim_end(), parse(&input).to_string())
    }


    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_grid::<13>(INPUT)), 102);
    }


    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_grid::<13>(INPUT)), 94);
    }
}
