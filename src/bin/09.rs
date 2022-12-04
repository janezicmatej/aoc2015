use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

fn get_all_distances(input: &str) -> Vec<u32> {
    let mut distances = HashMap::new();
    let mut cities = HashSet::new();

    for line in input.trim().split('\n') {
        let (a, b) = line.split(" = ").tuple_windows().next().unwrap();
        let (x, y) = a.split(" to ").tuple_windows().next().unwrap();
        distances.insert((x.to_string(), y.to_string()), b.parse::<u32>().unwrap());
        distances.insert((y.to_string(), x.to_string()), b.parse::<u32>().unwrap());
        cities.insert(x.to_string());
        cities.insert(y.to_string());
    }

    cities
        .iter()
        .permutations(cities.len())
        .map(|x| {
            x.iter()
                .tuple_windows()
                .map(|(a, b)| distances.get(&(a.to_string(), b.to_string())).unwrap())
                .sum()
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    get_all_distances(input).iter().min().copied()
}
pub fn part_two(input: &str) -> Option<u32> {
    get_all_distances(input).iter().max().copied()
}
fn main() {
    let input = &aoc::read_file("inputs", 9);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 9);
        assert_eq!(part_one(&input), Some(605));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 9);
        assert_eq!(part_two(&input), Some(982));
    }
}
