use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use Gate::*;
use Operand::*;

#[derive(Debug, Eq, PartialEq, Hash)]
enum Operand {
    Address(String),
    Literal(u16),
}

impl Operand {
    fn get(&self, map: &HashMap<String, u16>) -> u16 {
        match self {
            Address(x) => *map.get(x).unwrap(),
            Literal(x) => *x,
        }
    }
}

impl From<&str> for Operand {
    fn from(value: &str) -> Self {
        match value.parse() {
            Ok(x) => Literal(x),
            Err(_) => Address(value.to_string()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Gate {
    Write(Operand, Operand),
    And(Operand, Operand, Operand),
    Or(Operand, Operand, Operand),
    LShift(Operand, u8, Operand),
    RShift(Operand, u8, Operand),
    Not(Operand, Operand),
}

impl Gate {
    fn can_evaluate(&self, map: &HashMap<String, u16>) -> bool {
        match self {
            Write(Address(x), _) => map.contains_key(x),
            Not(Address(x), _) => map.contains_key(x),
            LShift(Address(x), _, _) => map.contains_key(x),
            RShift(Address(x), _, _) => map.contains_key(x),
            And(x, y, _) | Or(x, y, _) => {
                let mut allow = true;
                if let Address(xx) = x {
                    allow &= map.contains_key(xx);
                };
                if let Address(yy) = y {
                    allow &= map.contains_key(yy);
                };
                allow
            }
            _ => true,
        }
    }
}

impl From<&str> for Gate {
    fn from(value: &str) -> Self {
        lazy_static! {
            static ref RE_SIGNAL: Regex = Regex::new(r"^(\w*) -> (\w*)$").unwrap();
            static ref RE_NOT: Regex = Regex::new(r"NOT (\w*) -> (\w*)").unwrap();
            static ref RE_GATE: Regex =
                Regex::new(r"^(\w*) (AND|OR|LSHIFT|RSHIFT) (\w*) -> (\w*)$").unwrap();
        }

        if let Some(cap) = RE_SIGNAL.captures(value) {
            return Write(Operand::from(&cap[1]), Operand::from(&cap[2]));
        };

        if let Some(cap) = RE_NOT.captures(value) {
            return Not(Operand::from(&cap[1]), Operand::from(&cap[2]));
        };

        if let Some(cap) = RE_GATE.captures(value) {
            match &cap[2] {
                "AND" => {
                    return And(
                        Operand::from(&cap[1]),
                        Operand::from(&cap[3]),
                        Operand::from(&cap[4]),
                    )
                }
                "OR" => {
                    return Or(
                        Operand::from(&cap[1]),
                        Operand::from(&cap[3]),
                        Operand::from(&cap[4]),
                    )
                }
                "LSHIFT" => {
                    return LShift(
                        Operand::from(&cap[1]),
                        cap[3].parse().unwrap(),
                        Operand::from(&cap[4]),
                    );
                }
                "RSHIFT" => {
                    return RShift(
                        Operand::from(&cap[1]),
                        cap[3].parse().unwrap(),
                        Operand::from(&cap[4]),
                    );
                }
                _ => unreachable!(),
            }
        };
        unreachable!()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: HashMap<String, u16> = HashMap::new();
    let mut set: HashSet<Gate> = HashSet::new();
    for line in input.trim().split('\n') {
        set.insert(Gate::from(line));
    }
    while !set.is_empty() {
        for gate in set.drain_filter(|x| x.can_evaluate(&map)).collect_vec() {
            match gate {
                Write(a, Address(b)) => map.insert(b.to_string(), a.get(&map)),
                And(a, b, Address(c)) => map.insert(c.to_string(), a.get(&map) & b.get(&map)),
                Or(a, b, Address(c)) => map.insert(c.to_string(), a.get(&map) | b.get(&map)),
                LShift(a, b, Address(c)) => map.insert(c.to_string(), a.get(&map) << b),
                RShift(a, b, Address(c)) => map.insert(c.to_string(), a.get(&map) >> b),
                Not(a, Address(b)) => map.insert(b.to_string(), !a.get(&map)),
                _ => unreachable!(),
            };
        }
    }
    Some(*map.get("a").unwrap() as u32)
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut map: HashMap<String, u16> = HashMap::new();
    let mut set: HashSet<Gate> = HashSet::new();
    for line in input.trim().split('\n') {
        let g = Gate::from(line);
        if let Write(a, Address(b)) = g {
            if b == "b" {
                map.insert("b".to_string(), 46065);
            } else {
                set.insert(Write(a, Address(b)));
            }
        } else {
            set.insert(g);
        }
    }
    while !set.is_empty() {
        for gate in set.drain_filter(|x| x.can_evaluate(&map)).collect_vec() {
            match gate {
                Write(a, Address(b)) => map.insert(b.to_string(), a.get(&map)),
                And(a, b, Address(c)) => map.insert(c.to_string(), a.get(&map) & b.get(&map)),
                Or(a, b, Address(c)) => map.insert(c.to_string(), a.get(&map) | b.get(&map)),
                LShift(a, b, Address(c)) => map.insert(c.to_string(), a.get(&map) << b),
                RShift(a, b, Address(c)) => map.insert(c.to_string(), a.get(&map) >> b),
                Not(a, Address(b)) => map.insert(b.to_string(), !a.get(&map)),
                _ => unreachable!(),
            };
        }
    }
    Some(*map.get("a").unwrap() as u32)
}
fn main() {
    let input = &aoc::read_file("inputs", 7);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 7);
        assert_eq!(part_one(&input), Some(65533));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 7);
        assert_eq!(part_two(&input), Some(19470));
    }
}
