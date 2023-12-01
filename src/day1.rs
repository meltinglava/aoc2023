use aoc_runner_derive::{aoc, aoc_generator};

const NUMBERS: [(&str, usize); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(str::to_string).collect()
}

fn first_number<T: Iterator<Item = char>>(mut input: T) -> usize {
    input
        .find(char::is_ascii_digit)
        .unwrap()
        .to_digit(10)
        .unwrap() as usize
}

fn find_number(input: &str) -> usize {
    let mut first = Vec::new();
    let mut last = Vec::new();
    for n in NUMBERS {
        if let Some(f) = input.find(n.0) {
            first.push((f, n.1));
        }
        if let Some(l) = input.rfind(n.0) {
            last.push((l, n.1));
        }
    }
    if let Some(first_digit) = input
        .chars()
        .enumerate()
        .find(|n| n.1.is_ascii_digit())
        .map(|(index, value)| (index, value.to_digit(10).unwrap() as usize))
    {
        first.push(first_digit);
    }
    if let Some(last_digit) = input
        .chars()
        .rev()
        .enumerate()
        .find(|n| n.1.is_ascii_digit())
        .map(|(index, value)| {
            (
                input.len() - 1 - index,
                value.to_digit(10).unwrap() as usize,
            )
        })
    {
        last.push(last_digit);
    }
    first.into_iter().min_by_key(|(index, _)| *index).unwrap().1
        * 10
        + last.into_iter().max_by_key(|(index, _)| *index).unwrap().1
}

#[aoc(day1, part1)]
fn part1(input: &[String]) -> usize {
    input
        .into_iter()
        .map(|n| first_number(n.chars()) * 10 + first_number(n.chars().rev()))
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &[String]) -> usize {
    input.iter().map(|n| n.as_str()).map(find_number).sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_p2_hole() {
        let lines = indoc! {"
            two1nine
            eightwothree
            Abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "};
        assert_eq!(part2(&input_generator(lines)), 281)
    }

    #[test]
    fn test_p2_parts() {
        let parts = [
            ("two1nine", 29),
            ("eightwothree", 83),
            ("Abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
        ];
        for (word, value) in parts {
            assert_eq!(
                find_number(word),
                value,
                "{} was not translated correctly",
                word
            );
        }
    }
}
