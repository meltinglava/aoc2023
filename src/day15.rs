use aoc_runner_derive::{aoc, aoc_generator};

type Hashmap<'a> = Vec<Vec<(&'a str, usize)>>;

fn hash_string(s: &str) -> u8 {
    s.bytes()
        .fold(0usize, |acc, c| (acc + c as usize) * 17 % 256) as u8
}

#[aoc_generator(day15)]
fn parse(input: &str) -> Vec<String> {
    input.split(',').map(str::to_owned).collect()
}

#[aoc(day15, part1)]
fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|s| hash_string(s))
        .map(Into::<usize>::into)
        .sum()
}

#[aoc(day15, part2)]
fn part2(input: &[String]) -> usize {
    let mut map: Hashmap = vec![Vec::new(); 256];
    for i in input {
        if i.contains('-') {
            let key = &i[..i.len() - 1];
            let hash = hash_string(key) as usize;
            map.get_mut(hash).unwrap().retain(|n| n.0 != key)
        } else if i.contains('=') {
            let (key, value) = i.split_once('=').unwrap();
            let value: usize = value.parse().unwrap();
            let hash = hash_string(key) as usize;
            let bucket = map.get_mut(hash).unwrap();
            if let Some(entry) = bucket.iter_mut().find(|n| n.0 == key) {
                entry.1 = value
            } else {
                bucket.push((key, value))
            }
        } else {
            unreachable!("Unknown instruction: {i}")
        }
    }
    map.into_iter()
        .enumerate()
        .flat_map(|(box_nr, bucket)| {
            bucket
                .into_iter()
                .enumerate()
                .map(move |(bucket_nr, item)| (box_nr + 1) * (bucket_nr + 1) * item.1)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn hash_hash() {
        assert_eq!(hash_string("HASH"), 52)
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 1320);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 145);
    }
}
