use md5::{Digest, Md5};

pub fn part_one(input: &str) -> Option<u32> {
    let key = input.trim().as_bytes();
    let mut hasher = Md5::new();
    for x in 0..std::u64::MAX {
        hasher.update(key);
        hasher.update(x.to_string().as_bytes());
        let output = hasher.finalize_reset();

        if output.starts_with(&[0, 0]) && output[2] <= 0x0F {
            return Some(x as u32);
        }
    }
    unreachable!()
}
pub fn part_two(input: &str) -> Option<u32> {
    let key = input.trim().as_bytes();
    let mut hasher = Md5::new();
    for x in 0..std::u64::MAX {
        hasher.update(key);
        hasher.update(x.to_string().as_bytes());
        let output = hasher.finalize_reset();

        if output.starts_with(&[0, 0, 0]) {
            return Some(x as u32);
        }
    }
    unreachable!()
}
fn main() {
    let input = &aoc::read_file("inputs", 4);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 4);
        assert_eq!(part_one(&input), Some(609043));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 4);
        assert_eq!(part_two(&input), Some(6742839));
    }
}
