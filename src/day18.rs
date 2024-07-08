use std::ops::Not;

use aoc_runner_derive::{aoc, aoc_generator};

use nom::{
    bytes::complete::{tag, take_while_m_n}, character::complete::{self, one_of}, combinator::map_res, multi::separated_list1, sequence::{delimited, pair, Tuple}, Finish, IResult, Parser
};

#[derive(Debug, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn from_hex_u8(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn from_hex_usize(input: &str) -> Result<usize, std::num::ParseIntError> {
    usize::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn hex_color_primary(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(2, 2, is_hex_digit),
        from_hex_u8
    ).parse(input)
}

fn hex_length_primary(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("#")(input)?;
    map_res(
        take_while_m_n(5, 5, is_hex_digit),
        from_hex_usize
    ).parse(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = (hex_color_primary, hex_color_primary, hex_color_primary).parse(input)?;
    Ok((input, Color { red, green, blue }))
}

fn direction(input: &str) -> IResult<&str, Direction> {
    let (input, letter) = one_of("UDLR0123")(input)?;
    let dir = match letter {
        'U' | '3' => North,
        'D' | '1' => South,
        'L' | '2' => West,
        'R' | '0' => East,
        _ => unreachable!(),
    };
    Ok((input, dir))
}

fn parse_line_part1(input: &str) -> IResult<&str, Digg> {
    let (input, (direction, _, length, _, _color)) = (direction, tag(" "), complete::u8, tag(" "), delimited(tag("("), hex_color, tag(")"))).parse(input)?;
    Ok((input, Digg{
        direction,
        length: length.into(),
        // color,
    }))
}

fn get_part2_length_direction(input: &str) -> IResult<&str, (usize, Direction)> {
    pair(hex_length_primary, direction)(input)
}

fn parse_line_part2(input: &str) -> IResult<&str, Digg> {
    let (input, (_direction, _, _old_length, _, (length, direction))) = (direction, tag(" "), complete::u8, tag(" "), delimited(tag("("), get_part2_length_direction, tag(")"))).parse(input)?;
    Ok((input, Digg{
        direction,
        length,
        //color,
    }))
}

type Coord = (isize, isize);

#[derive(Debug, PartialEq, Eq)]
struct Digg {
    direction: Direction,
    length: usize,
    //color: Color,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn step(&self, coord: Coord) -> Option<Coord> {
        Some(match self {
            North => (coord.0, coord.1.checked_sub(1)?),
            South => (coord.0, coord.1.checked_add(1)?),
            East => (coord.0.checked_add(1)?, coord.1),
            West => (coord.0.checked_sub(1)?, coord.1),
        })
    }

    fn steps(&self, coord: Coord, len: isize) -> Option<Coord> {
        Some(match self {
            North => (coord.0, coord.1.checked_sub(len)?),
            South => (coord.0, coord.1.checked_add(len)?),
            East => (coord.0.checked_add(len)?, coord.1),
            West => (coord.0.checked_sub(len)?, coord.1),
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

fn dig_and_fill(diggs: &[Digg]) -> usize {
    let mut trenches = Vec::<Coord>::new();
    let mut pos = (0, 0);
    let mut trenches_len = 0;
    trenches.push(pos);
    for digg in diggs {
        pos = digg.direction.steps(pos, digg.length as isize).unwrap();
        trenches.push(pos);
        trenches_len += digg.length;
    }
    let area = (trenches.windows(2).map(|a| a[0].0 * a[1].1 - a[0].1 * a[1].0).sum::<isize>()) as f64 / 2.;
    (area - ((trenches_len as f64) * 0.5)) as usize + 1 + trenches_len
}

fn parse_nom_part1(input: &str) -> IResult<&str, Vec<Digg>> {
    separated_list1(tag("\n"), parse_line_part1)(input)
}

fn parse_nom_part2(input: &str) -> IResult<&str, Vec<Digg>> {
    separated_list1(tag("\n"), parse_line_part2)(input)
}

#[aoc_generator(day18, part1)]
fn parse(input: &str) -> Vec<Digg> {
    parse_nom_part1(input).finish().unwrap().1
}

#[aoc(day18, part1)]
fn part1(diggs: &[Digg]) -> usize {
    dig_and_fill(diggs)
}

#[aoc_generator(day18, part2)]
fn parse_part2(input: &str) -> Vec<Digg> {
    parse_nom_part2(input).finish().unwrap().1
}

#[aoc(day18, part2)]
fn part2(diggs: &[Digg]) -> usize {
    dig_and_fill(diggs)
}

// #[aoc(day17, part2)]
// fn part2(diggs: &[Digg]) -> usize {
//     find_shortest_path(grid, possible_moves_by_ultra_cruciblescrucibles, 4..=10)
// }


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static INPUT: &str = indoc! {r"
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    "};


    #[test]
    fn test_parse_part1() {
        let parsed = parse(INPUT);
        assert_eq!(parsed.len(), 14);
        let first = Digg {
            direction: East,
            length: 6,
            //color: Color { red: 0x70, green: 0xc7, blue: 0x10 },
        };
        assert_eq!(parsed[0], first);
    }

    #[test]
    fn test_parse_part2() {
        let parsed = parse_part2(INPUT);
        assert_eq!(parsed.len(), 14);
        let first = Digg {
            direction: East,
            length: 461937,
            //color: Color { red: 0x70, green: 0xc7, blue: 0x10 },
        };
        assert_eq!(parsed[0], first);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 62);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_part2(INPUT)), 952408144115);
    }
}
