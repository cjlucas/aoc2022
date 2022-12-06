use std::collections::HashSet;

const INPUT: &'static str = include_str!("../../inputs/day06.txt");

fn find_marker(input: &str, marker_size: usize) -> u64 {
    let mut answer = 0;

    let chars: Vec<char> = input.chars().collect();

    for window in chars.windows(marker_size) {
        let set: HashSet<&char> = window.iter().collect();
        if set.len() == marker_size {
            return answer + marker_size as u64;
        }

        answer += 1;
    }

    unreachable!()
}

fn part1(input: &str) -> u64 {
    find_marker(input, 4)
}

fn part2(input: &str) -> u64 {
    find_marker(input, 14)
}

fn main() {
    println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01_part1_sample1() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    }

    #[test]
    fn test_day01_part1_sample2() {
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    }

    #[test]
    fn test_day01_part1_sample3() {
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    }

    #[test]
    fn test_day01_part1_sample4() {
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    }

    #[test]
    fn test_day01_part1_sample5() {
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_day01_part1() {
        assert_eq!(part1(INPUT), 1896);
    }

    #[test]
    fn test_day01_part2_sample1() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    }

    #[test]
    fn test_day01_part2_sample2() {
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    }

    #[test]
    fn test_day01_part2_sample3() {
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    }

    #[test]
    fn test_day01_part2_sample4() {
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    }

    #[test]
    fn test_day01_part2_sample5() {
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn test_day01_part2() {
        assert_eq!(part2(INPUT), 3452);
    }
}
