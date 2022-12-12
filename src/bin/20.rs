pub fn part_one(input: &str) -> Option<u32> {
    let limit = input.parse::<usize>().unwrap() / 10;
    let elves = 1_000_000;
    let mut delivered = vec![0; elves];
    for i in 1..elves {
        for j in (i..elves).step_by(i) {
            delivered[j] += i;
        }
    }

    Some(
        delivered
            .iter()
            .enumerate()
            .find(|(_, &x)| x > limit)
            .unwrap()
            .0 as u32,
    )
}
pub fn part_two(input: &str) -> Option<u32> {
    let limit = input.parse().unwrap();
    let elves = 1_000_000;
    let mut delivered = vec![0; elves];
    for i in 1..elves {
        for j in (i..elves).step_by(i).take(50) {
            delivered[j] += i * 11;
        }
    }

    Some(
        delivered
            .iter()
            .enumerate()
            .find(|(_, &x)| x > limit)
            .unwrap()
            .0 as u32,
    )
}
fn main() {
    let input = &aoc::read_file("inputs", 20);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
