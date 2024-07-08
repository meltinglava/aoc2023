use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

fn parse_distance(input: &str, distance: usize) -> Vec<(usize, usize)> {
    let mut seen = Vec::<(usize, usize)>::new();
    let mut x_seen = vec![false; input.lines().next().unwrap().len()];
    let mut extra_lines = 0;
    for (y, line) in input.lines().enumerate() {
        let mut items = 0;
        for (x, symbol) in line.char_indices() {
            match symbol {
                '#' => {
                    items += 1;
                    seen.push((x, y + extra_lines * (distance - 1)));
                    x_seen[x] = true;
                }
                '.' => (),
                c => unreachable!("Unknown symbol: {}", c),
            }
        }
        if items == 0 {
            extra_lines += 1;
        }
    }
    x_seen
        .into_iter()
        .enumerate()
        .rev()
        .filter(|(_, found)| !found)
        .for_each(|(pos, _)| {
            seen.iter_mut()
                .filter(|(x, _)| *x >= pos)
                .for_each(|(x, _)| *x += distance - 1);
        });
    seen
}

#[aoc_generator(day11, part1)]
fn parse_p1(input: &str) -> Vec<(usize, usize)> {
    parse_distance(input, 2)
}

#[aoc_generator(day11, part2)]
fn parse_p2(input: &str) -> Vec<(usize, usize)> {
    parse_distance(input, 1_000_000)
}

fn shortest_path(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn shortest_path_every_pair(graph: &[(usize, usize)]) -> usize {
    graph
        .iter()
        .copied()
        .tuple_combinations()
        .map(|(a, b)| shortest_path(a, b))
        .sum()
}

#[aoc(day11, part1)]
fn part1(graph: &[(usize, usize)]) -> usize {
    shortest_path_every_pair(graph)
}

#[aoc(day11, part2)]
fn part2(graph: &[(usize, usize)]) -> usize {
    shortest_path_every_pair(graph)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static INPUT: &str = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    #[test]
    fn test_weights() {}

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_p1(INPUT)), 374);
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(part2(&parse_distance(INPUT, 10)), 1030);
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(part2(&parse_distance(INPUT, 100)), 8410);
    }
}
