use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_MEM: Regex = Regex::new(r#"(\\\\|\\"|\\x.{2})"#).unwrap();
    static ref RE_ENCODE: Regex = Regex::new(r#"(\\|")"#).unwrap();
}

fn mem_size(line: &str) -> usize {
    RE_MEM.replace_all(line, "").len() + RE_MEM.captures_iter(line).count() - 2
}

fn encode_size(line: &str) -> usize {
    line.len() + RE_ENCODE.captures_iter(line).count() + 2
}

pub fn part_one(input: &str) -> Option<u32> {
    let res: usize = input
        .trim()
        .split('\n')
        .map(|x| x.len() - mem_size(x))
        .sum();
    Some(res as u32)
}
pub fn part_two(input: &str) -> Option<u32> {
    let res: usize = input
        .trim()
        .split('\n')
        .map(|x| encode_size(x) - x.len())
        .sum();
    Some(res as u32)
}
fn main() {
    let input = &aoc::read_file("inputs", 8);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 8);
        assert_eq!(part_one(&input), Some(12));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 8);
        assert_eq!(part_two(&input), Some(19));
    }
}
