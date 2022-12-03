use std::collections::HashSet;

const INPUT: &'static str = include_str!("../../inputs/day03.txt");

fn part1(input: &str) -> u64 {
    let mut total = 0;

    for line in input.lines() {
        let div = line.len() / 2;
        let first: HashSet<char> = line.chars().take(div).collect();
        let second: HashSet<char> = line.chars().skip(div).collect();
        let mut intersection: Vec<_> = first.intersection(&second).collect();
        intersection.sort();

        let c = *intersection.last().unwrap();
        let foo = if c.is_uppercase() { 38 } else { 96 };

        total += TryInto::<u32>::try_into(*c).unwrap() as u64 - foo
    }

    total
}

fn part2(input: &str) -> u64 {
    let mut total = 0;

    let lines: Vec<_> = input.lines().collect();

    for group in lines.chunks(3) {
        let first: HashSet<_> = group[0].chars().collect();
        let second: HashSet<_> = group[1].chars().collect();
        let third: HashSet<_> = group[2].chars().collect();

        let mut intersection: HashSet<_> = first.intersection(&second).cloned().collect();
        intersection = intersection.intersection(&third).cloned().collect();
        let mut intersection: Vec<_> = intersection.iter().collect();
        intersection.sort();

        let c = *intersection.last().unwrap();
        let foo = if c.is_uppercase() { 38 } else { 96 };

        total += TryInto::<u32>::try_into(*c).unwrap() as u64 - foo
    }

    total
}

fn main() {
    println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day03_sample.txt");

    #[test]
    fn test_day01_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 157);
    }

    #[test]
    fn test_day01() {
        assert_eq!(part1(INPUT), 7766);
    }

    #[test]
    fn test_day02_sample() {
        assert_eq!(part2(SAMPLE_INPUT), 70);
    }

    #[test]
    fn test_day02() {
        assert_eq!(part2(INPUT), 2415);
    }
}
