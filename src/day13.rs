use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use Material::*;

struct Grid {
    grid: Vec<Vec<Material>>,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|line| line.chars().map(Material::try_from).try_collect())
            .try_collect()
            .unwrap();
        Ok(Self { grid })
    }
}

impl Grid {
    fn invert(&self) -> Self {
        let mut inverted = vec![Vec::with_capacity(self.grid.len()); self.grid[0].len()];
        for i in 0..self.grid[0].len() {
            for grid_line in &self.grid {
                inverted[i].push(grid_line[i]);
            }
        }
        Self { grid: inverted }
    }

    fn horisontal(&self) -> Option<usize> {
        self.invert().vertical()
    }

    fn vertical(&self) -> Option<usize> {
        let mut centers: Vec<usize> = Vec::new();
        for (line_number, grid_line) in self.grid.iter().enumerate() {
            centers.retain(|center| {
                line_number
                    .checked_sub((line_number - center) * 2 + 1)
                    .map_or(true, |i| self.grid[i] == *grid_line)
            });
            if line_number != 0 && self.grid[line_number - 1] == *grid_line {
                centers.push(line_number);
            }
        }
        centers.sort_by_key(|n| (self.grid.len() / 2).abs_diff(*n));
        centers.first().copied()
    }

    fn smudge_horisontal(&self) -> Option<usize> {
        self.invert().smudge_vertical()
    }

    fn smudge_vertical(&self) -> Option<usize> {
        let mut centers: Vec<(usize, usize)> = Vec::new();
        for (line_number, grid_line) in self.grid.iter().enumerate() {
            centers.retain_mut(|center| {
                if let Some(found_line) = line_number.checked_sub((line_number - center.0) * 2 + 1)
                {
                    center.1 += differences_in_material(&self.grid[found_line], grid_line);
                    center.1 < 2
                } else {
                    true
                }
            });
            if line_number != 0 {
                let score = differences_in_material(&self.grid[line_number - 1], grid_line);
                if score < 2 {
                    centers.push((line_number, score));
                }
            }
        }
        centers
            .into_iter()
            .filter(|&(_, s)| s == 1)
            .map(|(n, _)| n)
            .sorted_by_key(|n| (self.grid.len() / 2).abs_diff(*n))
            .next()
    }

    fn mirror_score(&self) -> usize {
        self.vertical()
            .map(|n| n * 100)
            .or_else(|| self.horisontal())
            .unwrap()
    }

    fn smugde_mirror_score(&self) -> usize {
        self.smudge_vertical()
            .map(|n| n * 100)
            .or_else(|| self.smudge_horisontal())
            .unwrap()
    }
}

/// Takes 2 slices of materials and returns how many are different
fn differences_in_material(a: &[Material], b: &[Material]) -> usize {
    assert_eq!(a.len(), b.len());
    a.iter()
        .copied()
        .zip(b.iter().copied())
        .map(|(a, b)| usize::from(a != b))
        .sum()
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
enum Material {
    Ash,
    Rock,
}

impl TryFrom<char> for Material {
    type Error = char;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Ash),
            '#' => Ok(Rock),
            _ => Err(c),
        }
    }
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<Grid> {
    input
        .split("\n\n")
        .map(Grid::from_str)
        .try_collect()
        .unwrap()
}

#[aoc(day13, part1)]
fn part1(grid: &[Grid]) -> usize {
    grid.iter().map(Grid::mirror_score).sum()
}

#[aoc(day13, part2)]
fn part2(grid: &[Grid]) -> usize {
    grid.iter().map(Grid::smugde_mirror_score).sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static INPUT: &str = indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 405);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 400);
    }
}
