use itertools::Itertools;
use Instruction::*;
enum Register {
    A,
    B,
}

impl From<&str> for Register {
    fn from(value: &str) -> Self {
        match value {
            "a" => Register::A,
            "b" => Register::B,
            _ => unreachable!(),
        }
    }
}

enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    JumpOffset(i32),
    JumpEven(Register, i32),
    JumpOne(Register, i32),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        match value.split_once(' ').unwrap() {
            ("hlf", x) => Half(Register::from(x)),
            ("tpl", x) => Triple(Register::from(x)),
            ("inc", x) => Increment(Register::from(x)),
            ("jmp", x) => JumpOffset(x.parse().unwrap()),
            ("jie", x) => {
                let (reg, offset) = x.split_once(", ").unwrap();
                JumpEven(Register::from(reg), offset.parse().unwrap())
            }
            ("jio", x) => {
                let (reg, offset) = x.split_once(", ").unwrap();
                JumpOne(Register::from(reg), offset.parse().unwrap())
            }
            _ => unreachable!(),
        }
    }
}

fn run_instructions(input: &str, mut a: u32, mut b: u32) -> Option<u32> {
    let instructions = input.split('\n').map(Instruction::from).collect_vec();
    let mut index = 0;
    while let Some(instruction) = instructions.get(index) {
        match instruction {
            Half(x) => {
                match x {
                    Register::A => a /= 2,
                    Register::B => b /= 2,
                }
                index += 1;
            }
            Triple(x) => {
                match x {
                    Register::A => a *= 3,
                    Register::B => b *= 3,
                }
                index += 1;
            }
            Increment(x) => {
                match x {
                    Register::A => a += 1,
                    Register::B => b += 1,
                }
                index += 1;
            }
            JumpOffset(x) => index += *x as usize,
            JumpEven(r, x) => {
                let reg = match r {
                    Register::A => a,
                    Register::B => b,
                };
                if reg % 2 == 0 {
                    index += *x as usize
                } else {
                    index += 1;
                }
            }
            JumpOne(r, x) => {
                let reg = match r {
                    Register::A => a,
                    Register::B => b,
                };
                if reg == 1 {
                    index += *x as usize
                } else {
                    index += 1;
                }
            }
        }
    }
    Some(b)
}

pub fn part_one(input: &str) -> Option<u32> {
    run_instructions(input, 0, 0)
}
pub fn part_two(input: &str) -> Option<u32> {
    run_instructions(input, 1, 0)
}
fn main() {
    let input = &aoc::read_file("inputs", 23);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 23);
        assert_eq!(part_one(&input.trim()), Some(2));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 23);
        assert_eq!(part_two(&input.trim()), Some(2));
    }
}
