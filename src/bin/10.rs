fn process(s: &str) -> String {
    let mut new_s = String::new();
    let mut count = 0;
    let mut current = s.chars().next().unwrap();

    for c in s.chars() {
        if c == current {
            count += 1;
        } else {
            new_s.push_str(&count.to_string());
            new_s.push(current);
            current = c;
            count = 1
        }
    }

    new_s.push_str(&count.to_string());
    new_s.push(current);
    new_s
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut res = input.to_string();
    for _ in 0..40 {
        res = process(&res);
    }
    Some(res.len() as u32)
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut res = input.to_string();
    for _ in 0..50 {
        res = process(&res);
    }
    Some(res.len() as u32)
}
fn main() {
    let input = &aoc::read_file("inputs", 10);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 10);
        assert_eq!(part_one(&input), Some(124496));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 10);
        assert_eq!(part_two(&input), Some(1766402));
    }
}
