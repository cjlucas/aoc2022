use std::collections::HashSet;
use std::str::FromStr;

const INPUT: &'static str = include_str!("../../inputs/day06.txt");

fn part1(input: &str) -> u64 {
    let mut answer = 0;

    let chars: Vec<char> = input.chars().collect();

    for window in chars.windows(4) {
        let set: HashSet<&char> = window.iter().collect();
        if set.len() == 4 {
            return answer + 4;
        }

        answer += 1;
    }

    unreachable!()
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

    const SAMPLE_INPUT: &'static str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_day01_sample1() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    }

    #[test]
    fn test_day01_sample2() {
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    }

    #[test]
    fn test_day01_sample3() {
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    }

    #[test]
    fn test_day01_sample4() {
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    }

    #[test]
    fn test_day01_sample5() {
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_day01() {
        assert_eq!(part1(INPUT), 7766);
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
