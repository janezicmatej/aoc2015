use std::mem::swap;

use itertools::Itertools;

const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

fn count_surroundings(i: usize, j: usize, grid: &Vec<Vec<u32>>) -> u32 {
    let mut count = 0;
    for (di, dj) in DIRECTIONS.iter() {
        let (ii, jj) = (i as i32 + di, j as i32 + dj);
        if ii < 0 || jj < 0 || ii >= grid.len() as i32 || jj >= grid[0].len() as i32 {
            continue;
        }
        count += grid[ii as usize][jj as usize];
    }
    count
}

fn conways_lights(input: &str, force_on: &[(usize, usize)]) -> Option<u32> {
    let mut lights: Vec<Vec<u32>> = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| match y {
                    '.' => 0,
                    '#' => 1,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let mut new_lights = lights.to_vec();

    for _ in 0..100 {
        for (i, j) in force_on.iter() {
            lights[*i][*j] = 1;
        }
        for i in 0..lights.len() {
            for j in 0..lights[i].len() {
                match (lights[i][j], count_surroundings(i, j, &lights)) {
                    (1, 2) | (_, 3) => new_lights[i][j] = 1,
                    (_, _) => new_lights[i][j] = 0,
                };
            }
        }
        swap(&mut lights, &mut new_lights);
    }
    for (i, j) in force_on.iter() {
        lights[*i][*j] = 1;
    }
    Some(lights.iter().map(|x| x.iter().sum::<u32>()).sum())
}

pub fn part_one(input: &str) -> Option<u32> {
    conways_lights(input, &[])
}
pub fn part_two(input: &str) -> Option<u32> {
    let max = input.lines().collect_vec().len() - 1;
    conways_lights(input, &[(0, 0), (0, max), (max, 0), (max, max)])
}
fn main() {
    let input = &aoc::read_file("inputs", 18);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 18);
        assert_eq!(part_one(&input), Some(4));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 18);
        assert_eq!(part_two(&input), Some(7));
    }
}
