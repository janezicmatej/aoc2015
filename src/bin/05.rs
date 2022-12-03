use itertools::Itertools;
use lazy_static::lazy_static;
pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;

    lazy_static! {
        static ref VOWELS: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
    };

    lazy_static! {
        static ref PAIRS: Vec<(char, char)> = vec![('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];
    };

    for keyword in input.to_ascii_lowercase().split('\n') {
        let vowels = keyword.chars().filter(|c| VOWELS.contains(c)).count();
        if vowels < 3 {
            continue;
        };
        let mut double = false;
        let mut contains_forbidden = false;
        for (letter, next) in keyword.chars().tuple_windows() {
            contains_forbidden |= PAIRS.contains(&(letter, next));
            if letter == next {
                double = true;
            }
        }
        if double && !contains_forbidden {
            count += 1;
        };
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;

    for keyword in input.to_ascii_lowercase().split('\n') {
        let mut split_repeat = false;
        for (a, _, c) in keyword.chars().tuple_windows() {
            split_repeat |= a == c;
        }

        if !split_repeat {
            continue;
        };

        for i in 2..keyword.len() {
            if keyword[i..].contains(&keyword[i - 2..i]) {
                count += 1;
                break;
            }
        }
    }

    Some(count)
}
fn main() {
    let input = &aoc::read_file("inputs", 5);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 5);
        assert_eq!(part_one(&input), Some(1));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 5);
        assert_eq!(part_two(&input), Some(2));
    }
}
