pub fn part_one(input: &str) -> Option<u32> {
    let present_dimensions: Vec<Vec<u32>> = input
        .trim()
        .split('\n')
        .map(|x| x.split('x').map(|y| y.parse::<u32>().unwrap()).collect())
        .collect();

    let mut wrapping = 0;
    for present in present_dimensions.iter() {
        if let [x, y, z] = &present[..] {
            wrapping +=
                2 * (x * y + y * z + z * x) + vec![x * y, y * z, z * x].iter().min().unwrap();
        }
    }
    Some(wrapping)
}
pub fn part_two(input: &str) -> Option<u32> {
    let present_dimensions: Vec<Vec<u32>> = input
        .trim()
        .split('\n')
        .map(|x| x.split('x').map(|y| y.parse::<u32>().unwrap()).collect())
        .collect();

    let mut wrapping = 0;
    for present in present_dimensions.iter() {
        if let [x, y, z] = &present[..] {
            // ribbon
            wrapping += x * y * z;
            // ribbon base
            wrapping += 2 * (x + y + z - present.iter().max().unwrap());
        }
    }
    Some(wrapping)
}
fn main() {
    let input = &aoc::read_file("inputs", 2);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 2);
        assert_eq!(part_one(&input), Some(58));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 2);
        assert_eq!(part_two(&input), Some(34));
    }
}
