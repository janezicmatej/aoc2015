use lazy_static::lazy_static;
use regex::Regex;
use std::{iter::once, ops::Add};

lazy_static! {
    static ref RE: Regex = Regex::new(r#"-?\d+"#).unwrap();
}

#[derive(Debug, Clone, Copy)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl From<&str> for Ingredient {
    fn from(value: &str) -> Self {
        let mut finds = RE.find_iter(value).map(|x| x.as_str().parse().unwrap());

        Self {
            capacity: finds.next().unwrap(),
            durability: finds.next().unwrap(),
            flavor: finds.next().unwrap(),
            texture: finds.next().unwrap(),
            calories: finds.next().unwrap(),
        }
    }
}

impl Add for Ingredient {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            capacity: self.capacity + other.capacity,
            durability: self.durability + other.durability,
            flavor: self.flavor + other.flavor,
            texture: self.texture + other.texture,
            calories: self.calories + other.calories,
        }
    }
}

impl Ingredient {
    fn spoon(&self, spoons: i32) -> Ingredient {
        Ingredient {
            capacity: self.capacity * spoons,
            durability: self.durability * spoons,
            flavor: self.flavor * spoons,
            texture: self.texture * spoons,
            calories: self.calories * spoons,
        }
    }
    fn score(ingredients: &[Ingredient], partition: &[u32], skip: bool) -> u32 {
        let init = Ingredient {
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        };

        let last_index = 100 - partition.iter().sum::<u32>();

        let after = ingredients
            .iter()
            .zip(partition.iter().chain(once(&last_index)))
            .map(|(ingr, x)| ingr.spoon(*x as i32))
            .fold(init, |acc, x| acc + x);

        if vec![
            after.capacity,
            after.durability,
            after.flavor,
            after.texture,
        ]
        .iter()
        .any(|&x| x < 0)
        {
            return 0;
        }

        if skip && after.calories != 500 {
            return 0;
        }

        (after.capacity * after.durability * after.flavor * after.texture) as u32
    }
}

fn solve(input: &str, skip: bool) -> Option<u32> {
    let ingredients = Vec::from_iter(input.split('\n').map(Ingredient::from));
    let mut scores = Vec::new();

    // create a vector of length one less then ingredients as last one is
    // calculated with 100 - sum
    let mut partition = vec![0; ingredients.len() - 1];
    // push 0, 0, ..., 0
    scores.push(Ingredient::score(&ingredients, &partition, skip));

    // increase vector indices like counting until last index == 100
    while partition.last().unwrap() != &100 {
        for idx in 0..partition.len() {
            if partition[idx..].iter().sum::<u32>() < 100 {
                // can increase this index
                partition[idx] += 1;
                break;
            } else {
                // found 100, reset this to 0 and increase next one
                partition[idx] = 0;
            }
        }
        // every time this for loop was broken we increased some index by one
        // and we can get the last index by doing 100 - sum
        scores.push(Ingredient::score(&ingredients, &partition, skip));
    }

    scores.iter().max().copied()
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, false)
}
pub fn part_two(input: &str) -> Option<u32> {
    solve(input, true)
}
fn main() {
    let input = &aoc::read_file("inputs", 15);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 15);
        assert_eq!(part_one(input.trim()), Some(62842880));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 15);
        assert_eq!(part_two(input.trim()), Some(57600000));
    }
}
