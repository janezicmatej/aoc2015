pub fn part_one(input: &str) -> Option<u32> {
    let count = input.matches("(").count() - input.matches(")").count();
    Some(count.try_into().unwrap())
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut floor = 0;
    for (idx, char) in input.chars().enumerate() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("oops"),
        };

        if floor < 0 {
            return Some(idx as u32 + 1);
        }
    }
    None
}
fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        // let input = aoc::read_file("test_inputs", 1);
        let input = "(()(()(";
        assert_eq!(part_one(&input), Some(3));
    }
    #[test]
    fn test_part_two() {
        // let input = aoc::read_file("test_inputs", 1);
        let input = "()())";
        assert_eq!(part_two(&input), Some(5));
    }
}
