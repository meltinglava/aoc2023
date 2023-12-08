use std::collections::HashMap;

struct Map<'a> {
    movements: Vec<usize>, // 0 = Left, 1 = Right
    spots: HashMap<&'a str, [&'a str; 2]>,
}

impl<'a> TryFrom<&'a str> for Map<'a> {
    type Error = ();

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let (movements, spots) = s.split_once("\n\n").unwrap();
        let movements = movements
            .chars()
            .map(|c| match c {
                'L' => 0,
                'R' => 1,
                _ => unreachable!("Unknown movement: Known are L, and R. Got: {c}"),
            })
            .collect();

        let spots = spots
            .lines()
            .map(|line| {
                let key = &line[..3];
                let left = &line[7..10];
                let right = &line[12..15];
                (key, [left, right])
            })
            .collect();

        Ok(Self { movements, spots })
    }
}

impl<'a> Map<'a> {
    fn walk(&self, start: &str, end: &str) -> usize {
        let mut current_spot = self.spots.get(start).unwrap();
        let mut last_key = None;
        self.movements
            .iter()
            .cycle()
            .position(|&movement| {
                if last_key.map(|c: &str| c.ends_with(end)).unwrap_or(false) {
                    true
                } else {
                    let key = current_spot[movement];
                    current_spot = self.spots.get(key).unwrap();
                    last_key = Some(key);
                    false
                }
            })
            .expect("Empty movement")
    }

    fn ghost_walk(&self, start: &str, end: &str) -> usize {
        self.spots
            .keys()
            .cloned()
            .filter(|c| c.ends_with(start))
            .map(|s| self.walk(s, end))
            .reduce(num::integer::lcm)
            .expect("No keys starts with A")
    }
}

// https://github.com/gobanos/cargo-aoc/issues/20
// #[aoc_generator(day8)]
// fn parse<'a>(input: &'a str) -> Map<'a> {
//     input.parse().unwrap()
// }

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    Map::try_from(input).unwrap().walk("AAA", "ZZZ")
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    Map::try_from(input).unwrap().ghost_walk("A", "Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static INPUT1: &str = indoc! {"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    "};

    static INPUT2: &str = indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "};

    static INPUT3: &str = indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};

    #[test]
    fn part1_example_1() {
        assert_eq!(part1(INPUT1), 2);
    }

    #[test]
    fn part1_example_2() {
        assert_eq!(part1(INPUT2), 6);
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(part2(INPUT3), 6);
    }
}
