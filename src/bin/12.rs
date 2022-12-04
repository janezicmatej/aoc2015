use json::{parse, JsonValue::*, JsonValue};

fn parse_add(json_object: &JsonValue, skip_red: bool) -> i32 {
    match json_object {
        Number(x) => x.to_string().parse::<i32>().unwrap_or(0),
        Object(x) => {
            if skip_red {
                for (_, node) in x.iter() {
                    if let Short { .. } = node {
                        if node.as_str() == Some("red") {
                            return 0;
                        }
                    }
                }
            }
            x.iter().map(|(_, x)| parse_add(x, skip_red)).sum()
        }
        Array(x) => x.iter().map(|y| parse_add(y, skip_red)).sum(),
        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse_add(&parse(input).unwrap(), false) as u32)
}
pub fn part_two(input: &str) -> Option<u32> {
    Some(parse_add(&parse(input).unwrap(), true) as u32)
}
fn main() {
    let input = &aoc::read_file("inputs", 12);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 12);
        assert_eq!(part_one(&input.trim()), Some(15));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 12);
        assert_eq!(part_two(&input.trim()), Some(0));
    }
}
