use std::{iter::FromIterator, num::ParseIntError, ops::Range, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

struct Almanac {
    seeds: Vec<usize>,
    seeds_ranges: Vec<Range<usize>>,
    mappings: Vec<Mapping>,
}

struct Mapping {
    _source: String,
    _destination: String,
    mappings: Vec<Map>,
}

#[derive(Debug)]
struct Map {
    dest_range: Range<usize>,
    source_range: Range<usize>,
}

impl FromStr for Almanac {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut str_maps = input.split("\n\n");
        let seeds: Vec<_> =
            str_numbers_to_collecatble(str_maps.next().unwrap().split_once(": ").unwrap().1)?;

        let seeds_ranges = seeds.chunks(2).map(|c| c[0]..(c[0] + c[1])).collect();

        let mappings = str_maps
            .map(Mapping::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            seeds,
            seeds_ranges,
            mappings,
        })
    }
}

impl FromStr for Mapping {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let header = lines.next().unwrap();
        let (source, destination) = header
            .strip_suffix(" map:")
            .unwrap()
            .split_once("-to-")
            .unwrap();
        let mut mappings = lines.map(Map::from_str).collect::<Result<Vec<_>, _>>()?;
        mappings.sort_by_key(|map| map.source_range.start);
        Ok(Self {
            _source: source.to_string(),
            _destination: destination.to_string(),
            mappings,
        })
    }
}

impl FromStr for Map {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<_> = str_numbers_to_collecatble(s)?;
        assert_eq!(values.len(), 3);
        Ok(Self {
            dest_range: values[0]..(values[0] + values[2]),
            source_range: values[1]..(values[1] + values[2]),
        })
    }
}

impl Almanac {
    fn map_all_seeds(&self) -> Vec<usize> {
        self.seeds
            .iter()
            .copied()
            .map(|mut n| {
                for map in &self.mappings {
                    n = map.map_source_to_dest(n);
                }
                n
            })
            .collect()
    }
    fn min_of_all_seeds_ranges(&self) -> usize {
        self.seeds_ranges
            .iter()
            .cloned()
            .map(|seed_range| {
                let mut current_seed_ranges = vec![seed_range];
                for map in &self.mappings {
                    let mut next_ranges = Vec::new();
                    for seed_range in current_seed_ranges {
                        next_ranges.extend(map.map_source_range_to_dest_ranges(seed_range));
                    }
                    current_seed_ranges = next_ranges;
                }
                current_seed_ranges
                    .into_iter()
                    .map(|seed_range| seed_range.start)
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap()
    }
}

impl Mapping {
    fn map_source_to_dest(&self, source: usize) -> usize {
        self.mappings
            .iter()
            .find(|map| (map.source_range.start..(map.source_range.end)).contains(&source))
            .map_or(source, |map| source - map.source_range.start + map.dest_range.start)
    }

    fn map_source_range_to_dest_ranges(&self, mut range: Range<usize>) -> Vec<Range<usize>> {
        let mut ranges = Vec::new();
        for map in &self.mappings {
            if range.end < map.source_range.start {
                // remaning range ends before current range and as this is the
                // lowest source range avaliblle we add the remaing range to
                // ranges and break the loop
                break;
            } else if range.start >= map.source_range.end {
                // do nothing as the remaining range is larger than the mapping range
                continue;
            }
            if range.start < map.source_range.start {
                ranges.push(range.start..map.source_range.start);
                range = map.source_range.start..range.end;
            }
            let mapped_range_stop = range.end.min(map.source_range.end);
            let mapped_len = mapped_range_stop - range.start;
            let offset = range.start - map.source_range.start;
            ranges.push(
                (offset + map.dest_range.start)..(map.dest_range.start + offset + mapped_len),
            );
            if range.end <= map.source_range.end {
                return ranges;
            }
            range = map.source_range.end..range.end;
        }
        ranges.push(range);
        ranges
    }
}

fn str_numbers_to_collecatble<T>(s: &str) -> Result<T, ParseIntError>
where
    T: FromIterator<usize>,
{
    s.split(' ')
        .filter(|n| !n.is_empty())
        .map(usize::from_str)
        .collect()
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Almanac {
    input.parse().unwrap()
}

#[aoc(day5, part1)]
fn part1(input: &Almanac) -> usize {
    input.map_all_seeds().into_iter().min().unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &Almanac) -> usize {
    input.min_of_all_seeds_ranges()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static INPUT: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 35);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 46)
    }

    #[test]
    fn part2_best_case_one() {
        let mut input = parse(INPUT);
        input.seeds_ranges = vec![(82..83)];
        assert_eq!(part2(&input), 46);
    }

    #[test]
    fn part2_best_case_range() {
        let mut input = parse(INPUT);
        input.seeds_ranges = vec![(81..83)];
        assert_eq!(part2(&input), 46);
    }

    // #[test]
    // fn part2_by_part1() {
    //     let mut input = parse(include_str!("../input/2023/day5.txt"));
    //     input.seeds = (2637529854..2637529854 + 223394899).into_iter().collect_vec();
    //     assert_eq!(part1(&input), 46);
    // }
}
