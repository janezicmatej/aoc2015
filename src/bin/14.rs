use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r#"(?P<speed>\d+) km/s for (?P<duration>\d+) seconds, but then must rest for (?P<rest>\d+)"#
    )
    .unwrap();
}

const DISTANCE: u32 = 2503;

struct Reindeer {
    speed: u32,
    duration: u32,
    rest: u32,
}

impl Reindeer {
    fn cycle(&self) -> u32 {
        self.duration + self.rest
    }

    fn running(&self, traveled: u32) -> bool {
        traveled % self.cycle() < self.duration
    }
}

impl From<&str> for Reindeer {
    fn from(value: &str) -> Self {
        let cap = RE.captures(value).unwrap();

        let speed = cap["speed"].parse().unwrap();
        let duration = cap["duration"].parse().unwrap();
        let rest = cap["rest"].parse().unwrap();

        Self {
            speed,
            duration,
            rest,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split('\n')
        .map(|x| Reindeer::from(x))
        .map(|y| {
            ((DISTANCE / y.cycle()) * y.duration
                + vec![DISTANCE % y.cycle(), y.duration].iter().min().unwrap())
                * y.speed
        })
        .max()
}

#[derive(Clone, Debug)]
struct Scoreboard {
    current_distance: u32,
    score: u32,
}

impl Scoreboard {
    fn empty() -> Self {
        Self {
            current_distance: 0,
            score: 0,
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let reindeers = input.split('\n').map(|x| Reindeer::from(x)).collect_vec();
    let mut scoreboards = vec![Scoreboard::empty(); reindeers.len()];

    for traveled in 0..DISTANCE {
        for (idx, r) in reindeers
            .iter()
            .enumerate()
            .filter(|(_, x)| x.running(traveled))
        {
            scoreboards[idx].current_distance += r.speed;
        }

        let win = scoreboards
            .iter()
            .map(|x| x.current_distance)
            .max()
            .unwrap();

        for r in scoreboards.iter_mut().filter(|x| x.current_distance == win) {
            r.score += 1;
        }
    }
    scoreboards.iter().map(|x| x.score).max()
}

fn main() {
    let input = &aoc::read_file("inputs", 14);
    RE.captures(input);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 14);
        assert_eq!(part_one(&input.trim()), Some(1120));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 14);
        assert_eq!(part_two(&input.trim()), None);
    }
}
