use std::collections::HashSet;
use std::convert::Infallible;
use std::str::FromStr;

const INPUT: &'static str = include_str!("../../inputs/day04.txt");

fn part1(input: &str) -> u64 {
    let mut answer = 0;

    for line in input.lines() {
        let pair = line.split_once(',').unwrap();
        let first = pair.0.split_once('-').unwrap();
        let start1 = first.0.parse::<u64>().unwrap();
        let end1 = first.1.parse::<u64>().unwrap();

        let first = pair.1.split_once('-').unwrap();
        let start2 = first.0.parse::<u64>().unwrap();
        let end2 = first.1.parse::<u64>().unwrap();

        if (start1 >= start2 && end1 <= end2) || (start2 >= start1 && end2 <= end1) {
            answer += 1;
        }
    }

    answer
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

    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day04_sample.txt");

    #[test]
    fn test_day01_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 2);
    }

    #[test]
    fn test_day01() {
        assert_eq!(part1(INPUT), 448);
    }

    // #[test]
    // fn test_day02_sample() {
    //     assert_eq!(part2(SAMPLE_INPUT), 70);
    // }

    // #[test]
    // fn test_day02() {
    //     assert_eq!(part2(INPUT), 2415);
    // }
}
