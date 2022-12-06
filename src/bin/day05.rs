use std::collections::HashSet;
use std::convert::Infallible;
use std::str::FromStr;

const INPUT: &'static str = include_str!("../../inputs/day05.txt");

fn part1(input: &str) -> u64 {
    for line in input.lines() {
        let foo: Vec<_> = line.split(" ").collect();
        dbg!(foo);
    }

    0
}

fn part2(input: &str) -> u64 {
    0
}

fn main() {
    println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day05_sample.txt");

    #[test]
    fn test_day01_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 157);
    }

    // #[test]
    // fn test_day01() {
    //     assert_eq!(part1(INPUT), 7766);
    // }

    // #[test]
    // fn test_day02_sample() {
    //     assert_eq!(part2(SAMPLE_INPUT), 70);
    // }

    // #[test]
    // fn test_day02() {
    //     assert_eq!(part2(INPUT), 2415);
    // }
}
