use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref MFCSAM: HashMap<String, u32> = HashMap::from([
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1),
    ]);
}
pub fn part_one(input: &str) -> Option<u32> {
    for line in input.lines() {
        let (aunt, inventory) = line.split_once(": ").unwrap();
        let n = aunt[4..].parse().unwrap();
        let mut matching = true;
        for (item, amount_str) in inventory.split(", ").map(|x| x.split_once(": ").unwrap()) {
            let amount = amount_str.parse().unwrap();
            if MFCSAM.get(item).unwrap() != &amount {
                matching = false;
                break;
            }
        }
        if matching {
            return Some(n);
        }
    }
    None
}
pub fn part_two(input: &str) -> Option<u32> {
    for line in input.lines() {
        let (aunt, inventory) = line.split_once(": ").unwrap();
        let n = aunt[4..].parse().unwrap();
        let mut matching = true;
        for (item, amount_str) in inventory.split(", ").map(|x| x.split_once(": ").unwrap()) {
            let amount = amount_str.parse().unwrap();
            match item {
                "cats" | "trees" => {
                    if MFCSAM.get(item).unwrap() >= &amount {
                        matching = false;
                        break;
                    }
                }
                "pomeranians" | "goldfish" => {
                    if MFCSAM.get(item).unwrap() <= &amount {
                        matching = false;
                        break;
                    }
                }
                _ => {
                    if MFCSAM.get(item).unwrap() != &amount {
                        matching = false;
                        break;
                    }
                }
            }
        }
        if matching {
            return Some(n);
        }
    }
    None
}
fn main() {
    let input = &aoc::read_file("inputs", 16);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
