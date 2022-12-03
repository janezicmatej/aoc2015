use hashbrown::HashSet;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut visited = HashSet::with_capacity(input.len());
    visited.insert((0, 0));
    let mut x = 0;
    let mut y = 0;
    for direction in input.trim().chars() {
        match direction {
            '>' => x += 1,
            '<' => x -= 1,
            '^' => y += 1,
            'v' => y -= 1,
            _ => panic!("oops"),
        };
        visited.insert((x, y));
    }
    Some(visited.len() as u32)
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut visited = HashSet::with_capacity(input.len());
    visited.insert((0, 0));
    let (mut x1, mut y1, mut x2, mut y2) = (0, 0, 0, 0);
    for (santa, robot) in input.trim().chars().tuple_windows().step_by(2) {
        match santa {
            '>' => x1 += 1,
            '<' => x1 -= 1,
            '^' => y1 += 1,
            'v' => y1 -= 1,
            _ => panic!("oops"),
        };
        visited.insert((x1, y1));
        match robot {
            '>' => x2 += 1,
            '<' => x2 -= 1,
            '^' => y2 += 1,
            'v' => y2 -= 1,
            _ => panic!("oops"),
        };
        visited.insert((x2, y2));
    }
    Some(visited.len() as u32)
}
fn main() {
    let input = &aoc::read_file("inputs", 3);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 3);
        assert_eq!(part_one(&input), Some(2));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 3);
        assert_eq!(part_two(&input), Some(11));
    }
}
