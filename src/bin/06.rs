use lazy_static::lazy_static;
use regex::Regex;
use Instruction::*;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl FromIterator<usize> for Point {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut it = iter.into_iter();
        Point {
            x: it.next().unwrap(),
            y: it.next().unwrap(),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    On,
    Off,
    Toggle,
}

#[derive(Debug)]
struct Command {
    instruction: Instruction,
    from: Point,
    to: Point,
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(turn on|turn off|toggle) (\d{1,3},\d{1,3}) through (\d{1,3},\d{1, 3})"
            )
            .unwrap();
        }

        let capture = RE.captures(value).unwrap();

        let instruction = match capture[1].as_ref() {
            "turn on" => On,
            "turn off" => Off,
            "toggle" => Toggle,
            _ => unreachable!(),
        };

        let from: Point = capture[2].split(',').map(|x| x.parse().unwrap()).collect();

        let to: Point = capture[3].split(',').map(|x| x.parse().unwrap()).collect();

        Command {
            instruction,
            from,
            to,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut a = [[0; 1000]; 1000];
    for line in input.trim().split('\n') {
        let Command {
            instruction,
            from,
            to,
        } = Command::from(line);
        for x in from.x..=to.x {
            for y in from.y..=to.y {
                match instruction {
                    On => a[x][y] = 1,
                    Off => a[x][y] = 0,
                    Toggle => a[x][y] = 1 - a[x][y],
                }
            }
        }
    }
    Some(a.iter().map(|x| -> u32 { x.iter().sum() }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut a = [[0; 1000]; 1000];
    for line in input.trim().split('\n') {
        let Command {
            instruction,
            from,
            to,
        } = Command::from(line);
        for x in from.x..=to.x {
            for y in from.y..=to.y {
                match instruction {
                    On => a[x][y] = 1,
                    Off if a[x][y] > 0 => a[x][y] -= 1,
                    Off => (),
                    Toggle => a[x][y] += 2,
                }
            }
        }
    }
    Some(a.iter().map(|x| -> u32 { x.iter().sum() }).sum())
}
fn main() {
    let input = &aoc::read_file("inputs", 6);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 6);
        assert_eq!(part_one(&input), Some(998996));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 6);
        assert_eq!(part_two(&input), Some(1001996));
    }
}
