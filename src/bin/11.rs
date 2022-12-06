use aoc::helpers::ASCII_LOWERCASE;
use itertools::Itertools;

fn allowed_password(password: &str) -> bool {
    if password
        .chars()
        .filter(|x| vec!['i', 'o', 'l'].contains(x))
        .count()
        > 0
    {
        return false;
    }

    if password
        .chars()
        .tuple_windows()
        .map(|(a, b, c)| ASCII_LOWERCASE.contains(&format!("{a}{b}{c}")))
        .filter(|&y| y)
        .count()
        == 0
    {
        return false;
    }

    if ASCII_LOWERCASE
        .chars()
        .map(|a| password.contains(&format!("{a}{a}")))
        .filter(|&y| y)
        .count()
        < 2
    {
        return false;
    }

    true
}

fn next_password(password: &str) -> String {
    let mut next = String::new();
    let mut increase = true;

    for c in password.chars().rev() {
        match (c, increase) {
            ('z', true) => next.push('a'),
            (_, false) => next.push(c),
            (_, true) => {
                next.push((c as u8 + 1) as char);
                increase = false
            }
        }
    }
    if increase {
        next.push('a');
    }
    next = next.chars().rev().collect();

    if next
        .chars()
        .filter(|x| vec!['i', 'o', 'l'].contains(x))
        .count()
        == 0
    {
        return next;
    }

    let mut nnext = String::new();
    let mut found = false;
    for c in next.chars() {
        match (c, found) {
            (_, true) => nnext.push('a'),
            ('i' | 'o' | 'l', false) => {
                found = true;
                nnext.push((c as u8 + 1) as char);
            }
            (_, false) => nnext.push(c),
        }
    }
    nnext
}

pub fn part_one(input: &str) -> Option<String> {
    let mut password = input.to_string();
    while !allowed_password(&password) {
        password = next_password(&password);
    }
    Some(password)
}
pub fn part_two(input: &str) -> Option<String> {
    let mut password = input.to_string();
    while !allowed_password(&password) {
        password = next_password(&password);
    }
    password = next_password(&password);
    while !allowed_password(&password) {
        password = next_password(&password);
    }
    Some(password)
}
fn main() {
    let input = &aoc::read_file("inputs", 11);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 11);
        assert_eq!(part_one(input.trim()), Some("ghjaabcc".to_string()));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 11);
        assert_eq!(part_two(input.trim()), Some("ghjbbcdd".to_string()));
    }
}
