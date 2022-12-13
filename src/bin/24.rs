use aoc::helpers::to_vec;
use itertools::Itertools;

fn balance(gifts: &[u64], parts: u64) -> Option<u64> {
    let part = gifts.iter().sum::<u64>() / parts;

    // greedily search for smallest
    for i in 1..gifts.len() {
        for comb in gifts.iter().combinations(i) {
            if comb.iter().copied().sum::<u64>() == part {
                return Some(comb.iter().copied().product());
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let gifts = to_vec(input, '\n');
    balance(&gifts, 3)
}
pub fn part_two(input: &str) -> Option<u64> {
    let gifts = to_vec(input, '\n');
    balance(&gifts, 4)
}
fn main() {
    let input = &aoc::read_file("inputs", 24);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 24);
        assert_eq!(part_one(&input.trim()), Some(99));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 24);
        assert_eq!(part_two(&input.trim()), Some(44));
    }
}
