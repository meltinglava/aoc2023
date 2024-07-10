use std::{cmp::{max, Ordering}, collections::HashMap, iter::empty, mem::swap, ops::Range, sync::atomic::Ordering};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use nom::{bytes::complete::tag, character::complete::{self, alpha1, one_of}, combinator::{iterator, opt}, multi::separated_list1, sequence::{delimited, pair, terminated, tuple}, IResult, Parser};


#[derive(Debug)]
struct PartsOrganizer {
    workflows: HashMap<String, Vec<Flow>>,
    parts: Vec<Part>
}

impl PartsOrganizer {
    fn parse_parts(input: &str) -> IResult<&str, Self> {
        let mut it = iterator(input, terminated(parse_workflow, tag("\n")));
        let workflows = it.collect::<HashMap<String, Vec<Flow>>>();
        let input = it.finish()?.0;

        let (input, _) = tag("\n")(input)?;

        let (input, parts) = separated_list1(tag("\n"), Part::parse_part)(input)?;

        Ok((input, Self {
            workflows,
            parts
        }))
    }

    fn part1(&self) -> u64 {
        let ends = ["A", "R"];
        self
            .parts
            .iter()
            .filter(|part| {
                let mut key = "in".to_string();
                loop {
                    key = run_workflow(self.workflows.get(&key).unwrap(), part);
                    if ends.contains(&key.as_str()) {
                        return key == "A"
                    }
                }
            })
            .map(Part::score)
            .sum()
    }

    fn part2(&self) -> u64 {
        let mut stack: Vec<(String, XmasRanges)> = Vec::new();
        stack.push(("in".into(), XmasRanges::new()));
        let mut score = 0;
        while let Some((name, range)) = stack.pop() {
            if name == "A" {
                score += range.score()
            } else if name != "R" {
                self.run_flow_ranges(&name, range);
            }
        }
        score
    }

    fn run_flow_ranges(&self, name: &str, range: XmasRanges) -> impl Iterator<Item=(String, XmasRanges)> {
        let flow = self.workflows.get(name).unwrap();
        flow
            .into_iter();
        todo!();
        empty()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct XmasRanges {
    x: Range<u64>,
    m: Range<u64>,
    a: Range<u64>,
    s: Range<u64>,
}

impl XmasRanges {
    fn new() -> Self {
        Self{
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
    }

    fn get_range_by_rating(&self, rating: RatingsEnum) -> Range<u64> {
        match rating {
            RatingsEnum::X => self.x.clone(),
            RatingsEnum::M => self.m.clone(),
            RatingsEnum::A => self.a.clone(),
            RatingsEnum::S => self.s.clone(),
        }
    }

    fn score(self) -> u64 {
        (self.x.end - self.x.start)
            * (self.m.end - self.m.start)
            * (self.a.end - self.a.start)
            * (self.s.end - self.s.start)
    }

    fn new_from_range_with_rating(&self, range: Range<u64>, rating: RatingsEnum) -> Self {
        let mut out = self.clone();
        match rating {
            RatingsEnum::X => out.x = range,
            RatingsEnum::M => out.m = range,
            RatingsEnum::A => out.a = range,
            RatingsEnum::S => out.s = range,
        }
        out
    }

    fn insert_range_by_rating(&mut self, range: Range<u64>, rating: RatingsEnum) {
        match rating {
            RatingsEnum::X => self.x = range,
            RatingsEnum::M => self.m = range,
            RatingsEnum::A => self.a = range,
            RatingsEnum::S => self.s = range,
        }
    }

    fn split_off_range(&mut self, split: u64, rating: RatingsEnum, order: Ordering) -> Self {
        let rng = self.get_range_by_rating(rating);
        let mut new_rnges = (rng.start..split, split..rng.end);
        if order == Ordering::Less {
            swap(&mut new_rnges.0, &mut new_rnges.1);
        }
        self.insert_range_by_rating(new_rnges.0, rating);
        self.new_from_range_with_rating(new_rnges.1, rating)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn parse_part(input: &str) -> IResult<&str, Self> {
        let (input, (_, x, _, m, _, a, _, s)) =
            delimited(
                tag("{"),
                tuple((tag("x="), complete::u64, tag(",m="), complete::u64, tag(",a="), complete::u64, tag(",s="), complete::u64)),
                tag("}"))(input)?;
        Ok((input, Self{
            x,
            m,
            a,
            s,
        }))
    }

    fn get_rating(&self, rating: RatingsEnum) -> u64 {
        match rating {
            RatingsEnum::X => self.x,
            RatingsEnum::M => self.m,
            RatingsEnum::A => self.a,
            RatingsEnum::S => self.s,
        }
    }

    fn score(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum RatingsEnum {
    X,
    M,
    A,
    S,
}

impl RatingsEnum {
    fn parse_rating(input: &str) -> IResult<&str, Self> {
        let (input, c) = one_of("xmas")(input)?;
        let s = match c {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => unreachable!()
        };
        Ok((input, s))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Flow {
    condition: Option<Condition>,
    send_to: String,
}

impl Flow {
    fn parse_flow(input: &str) -> IResult<&str, Self> {
        let (input, (condition, send_to)) = pair(opt(Condition::parse_condition), alpha1)(input)?;
        Ok((input, Self{
            condition,
            send_to: send_to.to_owned(),
        }))
    }

    fn check(&self, part: &Part) -> Option<String> {
        match self.condition {
            Some(condition) => condition.check(part).then(|| self.send_to.clone()),
            None => Some(self.send_to.clone()),
        }
    }

    fn split_range(&self, range: &XmasRanges) -> Vec<XmasRanges> {
        self.condition.map_or_else(|| vec![range.clone()], |c| c.split_range(range.clone()))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Condition {
    rating: RatingsEnum,
    ordering: Ordering,
    value: u64,
}

fn parse_ordering(input: &str) -> IResult<&str, Ordering> {
    let (input, c) = one_of("<>")(input)?;
    let o = match c {
        '<' => Ordering::Less,
        '>' => Ordering::Greater,
        _ => unreachable!()
    };
    Ok((input, o))
}

impl Condition {
    fn parse_condition(input: &str) -> IResult<&str, Self> {
        let (input, (raiting, ordering, value)) = tuple((RatingsEnum::parse_rating, parse_ordering, terminated(complete::u64, tag(":")))).parse(input)?;
        Ok((input, Self{
            rating: raiting,
            ordering,
            value,
        }))
    }

    fn check(&self, part: &Part) -> bool {
        part.get_rating(self.rating).cmp(&self.value) == self.ordering
    }

    fn split_range(&self, range: XmasRanges) -> Vec<XmasRanges> {
        let r = range.get_range_by_rating(self.rating);
        let index = self.value + u64::from(self.ordering == Ordering::Greater);
    }
}

fn parse_workflow(input: &str) -> IResult<&str, (String, Vec<Flow>)> {
    tuple((get_name, delimited(tag("{"), separated_list1(tag(","), Flow::parse_flow), tag("}")))).parse(input)
}

fn get_name(input: &str) -> IResult<&str, String> {
    let (input, name) = alpha1(input)?;
    Ok((input, name.to_owned()))
}

fn run_workflow(workflow: &[Flow], part: &Part) -> String {
    workflow
        .iter()
        .find_map(|f| f.check(part))
        .unwrap()
}

#[aoc_generator(day19)]
fn parse(input: &str) -> PartsOrganizer {
    PartsOrganizer::parse_parts(input).unwrap().1
}

#[aoc(day19, part1)]
fn part1(parts: &PartsOrganizer) -> u64 {
    parts.part1()
}

#[aoc(day19, part2)]
fn part2(parts: &PartsOrganizer) -> u64 {
    parts.part2()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static INPUT: &str = indoc! {r"
    px{a<2006:qkq,m>2090:A,rfg}
    pv{a>1716:R,A}
    lnx{m>1548:A,A}
    rfg{s<537:gd,x>2440:R,A}
    qs{s>3448:A,lnx}
    qkq{x<1416:A,crn}
    crn{x>2662:A,R}
    in{s<1351:px,qqz}
    qqz{s>2770:qs,m<1801:hdj,R}
    gd{a>3333:R,R}
    hdj{m>838:A,pv}

    {x=787,m=2655,a=1222,s=2876}
    {x=1679,m=44,a=2067,s=496}
    {x=2036,m=264,a=79,s=2244}
    {x=2461,m=1339,a=466,s=291}
    {x=2127,m=1623,a=2188,s=1013}
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 19114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 167409079868000);
    }

    #[test]
    fn test_parts_parse() {
        let parsed = PartsOrganizer::parse_parts(INPUT);
        assert!(parsed.is_ok(), "{:?}", parsed);
        let parsed = parsed.unwrap().1;
        assert_eq!(parsed.workflows.len(), 11);
        assert_eq!(parsed.parts.len(), 5);
   }

    #[test]
    fn test_workflow_parse() {
        let input = "px{a<2006:qkq,rfg}";
        let flow_1 = Flow {
            condition: Some(Condition{
                rating: RatingsEnum::A,
                ordering: Ordering::Less,
                value: 2006,
            }),
            send_to: "qkq".to_string(),
        };
        let flow_2 = Flow {
            condition: None,
            send_to: "rfg".to_string(),
        };
        let expected = ("px".to_owned(), vec![flow_1, flow_2]);
        assert_eq!(parse_workflow(input).unwrap().1, expected);
    }

    #[test]
    fn test_flow_parse() {
        let input = "x>10:one";
        let expected = Flow {
            condition: Some(Condition{
                rating: RatingsEnum::X,
                ordering: Ordering::Greater,
                value: 10,
            }),
            send_to: "one".to_string(),
        };
        assert_eq!(Flow::parse_flow(input).unwrap().1, expected);
    }

    #[test]
    fn test_empty_parse() {
        let input = "one";
        let expected = Flow {
            condition: None,
            send_to: "one".to_string(),
        };
        assert_eq!(Flow::parse_flow(input).unwrap().1, expected);
    }

    #[test]
    fn test_condition_parse() {
        let input = "x>10:";
        let expected = Condition{
            rating: RatingsEnum::X,
            ordering: Ordering::Greater,
            value: 10,
        };
        assert_eq!(Condition::parse_condition(input).unwrap().1, expected);
    }

    #[test]
    fn test_raiting_parse() {
        let input = "x>10:";
        let expected = RatingsEnum::X;

        assert_eq!(RatingsEnum::parse_rating(input).unwrap().1, expected);
    }

    #[test]
    fn test_ordering_parse() {
        let input = ">10:";
        let expected = Ordering::Greater;

        assert_eq!(parse_ordering(input).unwrap().1, expected)
    }
}
