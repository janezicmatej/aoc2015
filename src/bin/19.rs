use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let (replacements, molecule) = input.split_once("\n\n")?;
    let mappings = replacements
        .split('\n')
        .map(|x| x.split_once(" => ").unwrap())
        .collect_vec();

    let mut molecules = HashSet::new();

    for (a, b) in mappings.iter() {
        let split = molecule.split(a).collect_vec();
        for i in 1..split.len() {
            let x = split[..i].join(a);
            let y = split[i..].join(a);
            molecules.insert(format!("{x}{b}{y}"));
        }
    }

    Some(molecules.len() as u32)
}

// https://www.reddit.com/r/adventofcode/comments/3xflz8/comment/cy4etju/
pub fn part_two(input: &str) -> Option<u32> {
    let (r, m) = input.split_once("\n\n")?;
    let mut molecule: String = m.chars().rev().collect();

    let mappings = r
        .split('\n')
        .map(|x| x.split_once(" => ").unwrap())
        .map(|(a, b)| {
            (
                b.chars().rev().collect::<String>(),
                a.chars().rev().collect::<String>(),
            )
        })
        .collect::<HashMap<String, String>>();

    let re = Regex::new(&mappings.iter().map(|(k, _)| k.as_str()).join("|")).unwrap();

    let mut count = 0;
    while molecule != "e" {
        let re_range = re.find(&molecule)?.range();
        let repl = &molecule[re_range];
        molecule = molecule.replacen(repl, mappings.get(repl)?, 1);
        count += 1;
    }

    Some(count)
}
fn main() {
    let input = &aoc::read_file("inputs", 19);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 19);
        assert_eq!(part_one(input.trim()), Some(4));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 19);
        assert_eq!(part_two(input.trim()), Some(3));
    }
}
