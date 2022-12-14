const ONE: u64 = 20151125;
const MULTIPLY: u64 = 252533;
const MOD: u64 = 33554393;

pub fn part_one(input: &str) -> Option<u64> {
    let l = input.len();
    let (row, column): (u32, u32) = input[80..l - 1]
        .split_once(", column ")
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .unwrap();

    let n = (row + column) * (row + column - 1) / 2 - row + 1;
    let mut c = 1;
    let mut code = ONE;
    while c < n {
        code = (code * MULTIPLY) % MOD;
        c += 1;
    }

    Some(code)
}
pub fn part_two(_input: &str) -> Option<u32> {
    None
}
fn main() {
    let input = &aoc::read_file("inputs", 25);
    aoc::solve!(1, part_one, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 25);
        assert_eq!(part_one(&input.trim()), Some(7726640));
    }
}
