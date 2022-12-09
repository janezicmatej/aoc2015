use aoc::helpers::to_vec;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let containers = to_vec::<u32>(input, '\n');
    Some(
        (0..containers.len())
            .map(|x| {
                containers
                    .iter()
                    .combinations(x)
                    .map(|y| y.iter().map(|w| *w).sum::<u32>())
                    .filter(|&z| z == 150)
                    .count()
            })
            .sum::<usize>() as u32,
    )
}
pub fn part_two(input: &str) -> Option<u32> {
    let containers = to_vec::<u32>(input, '\n');
    let v = (1..=containers.len())
        .map(|x| {
            containers
                .iter()
                .combinations(x)
                .map(|y| (y.len(), y.iter().map(|w| *w).sum::<u32>()))
                .filter(|(_, z)| z == &150)
                .map(|(t, _)| t)
                .collect_vec()
        })
        .filter(|x| x.len() > 0)
        .map(|x| {
            let m = x.iter().min().unwrap();
            (*m, x.iter().filter(|&y| y == m).count())
        })
        .collect_vec();
    let m = v.iter().map(|(a, _)| a).min().unwrap();
    Some(
        v.iter()
            .filter(|(a, _)| a == m)
            .map(|(_, b)| *b)
            .sum::<usize>() as u32,
    )
}

fn main() {
    let input = &aoc::read_file("inputs", 17);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
