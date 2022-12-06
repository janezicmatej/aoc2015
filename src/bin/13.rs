use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r#"(?P<person>\w+) would (?P<relation>\w{4}) (?P<amount>\d+) happiness units by sitting next to (?P<other>\w+)"#).unwrap();
}

#[derive(Debug, PartialEq, Hash)]
struct Line {
    person: String,
    other: String,
    amount: i32,
}

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        let cap = RE.captures(value).unwrap();

        let person = cap["person"].to_string();
        let other = cap["other"].to_string();
        let mut amount = cap["amount"].parse().unwrap();
        if let "lose" = &cap["relation"] {
            amount *= -1;
        };

        Line {
            person,
            other,
            amount,
        }
    }
}

fn solve(input: &str, insert_myself: bool) -> u32 {
    let mut map = HashMap::new();
    let mut set = HashSet::new();
    for line in input.split('\n') {
        let parsed = Line::from(line);
        map.insert(
            (parsed.person.to_string(), parsed.other.to_string()),
            parsed.amount,
        );
        set.insert(parsed.person.to_string());
    }

    if insert_myself {
        for p in set.iter() {
            map.insert((p.to_string(), "Matej".to_string()), 0);
            map.insert(("Matej".to_string(), p.to_string()), 0);
        }
        set.insert("Matej".to_string());
    }

    set.iter()
        .permutations(set.len())
        .map(|x| {
            x.iter()
                .cycle()
                .take(set.len() + 1)
                .tuple_windows()
                .map(|(a, b)| {
                    map.get(&(a.to_string(), b.to_string())).unwrap()
                        + map.get(&(b.to_string(), a.to_string())).unwrap()
                })
                .sum::<i32>()
        })
        .max()
        .unwrap() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, false))
}
pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, true))
}
fn main() {
    let input = &aoc::read_file("inputs", 13);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 13);
        assert_eq!(part_one(&input.trim()), Some(330));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 13);
        assert_eq!(part_two(&input.trim()), Some(286));
    }
}
