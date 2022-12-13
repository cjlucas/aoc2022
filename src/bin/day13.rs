use std::convert::Infallible;
use std::str::FromStr;

const INPUT: &'static str = include_str!("../../inputs/day13.txt");

#[derive(Debug, PartialEq)]
enum PacketData {
    List(Vec<PacketData>),
    Integer(u64),
}

impl FromStr for PacketData {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        dbg!(&s);
        let data = if s.chars().nth(0) == Some('[') {
            Self::from_str(&s[1..s.len() - 1]).unwrap()
        } else {
            Self::Integer(u64::from_str(s).unwrap())
        };

        Ok(data)
    }
}

fn part1(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().filter(|l| l.len() > 0).collect();

    for pair in lines.chunks(2) {
        dbg!(&pair);
        // let left = pair.next();
        // let right = pair.next();
    }

    dbg!(&lines);

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

    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day13_sample.txt");

    #[test]
    fn test_packet_data_from_str01() {
        assert_eq!(
            PacketData::from_str("[1,1,3,1,1]"),
            Ok(PacketData::List(vec![
                PacketData::Integer(1),
                PacketData::Integer(1),
                PacketData::Integer(3),
                PacketData::Integer(1),
                PacketData::Integer(1),
            ]))
        )
    }

    #[test]
    fn test_packet_data_from_str02() {
        assert_eq!(
            PacketData::from_str("[[1],[2,3,4]]"),
            Ok(PacketData::List(vec![
                PacketData::List(vec![PacketData::Integer(1),]),
                PacketData::List(vec![
                    PacketData::Integer(2),
                    PacketData::Integer(3),
                    PacketData::Integer(4),
                ]),
            ]))
        )
    }

    #[test]
    fn test_packet_data_from_str03() {
        assert_eq!(
            PacketData::from_str("[[[]]]"),
            Ok(PacketData::List(vec![PacketData::List(vec![
                PacketData::List(vec![])
            ])]))
        )
    }

    // #[test]
    // fn test_day01_sample() {
    //     assert_eq!(part1(SAMPLE_INPUT), 2);
    // }

    // #[test]
    // fn test_day01() {
    //     assert_eq!(part1(INPUT), 448);
    // }

    // #[test]
    // fn test_day02_sample() {
    //     assert_eq!(part2(SAMPLE_INPUT), 4);
    // }

    // #[test]
    // fn test_day02() {
    //     assert_eq!(part2(INPUT), 794);
    // }
}
