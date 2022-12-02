use std::str::FromStr;

const INPUT: &'static str = include_str!("../../inputs/day02.txt");

#[derive(Debug)]
struct UnknownInput;

#[derive(Debug)]
struct Round {
    my_shape: Shape,
    opp_shape: Shape,
}

impl Round {
    fn total_score(&self) -> u64 {
        let shape_score = match self.my_shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        let outcome_score = match (self.my_shape, self.opp_shape) {
            (Shape::Rock, Shape::Rock) => 3,
            (Shape::Rock, Shape::Paper) => 0,
            (Shape::Rock, Shape::Scissors) => 6,
            (Shape::Paper, Shape::Rock) => 6,
            (Shape::Paper, Shape::Paper) => 3,
            (Shape::Paper, Shape::Scissors) => 0,
            (Shape::Scissors, Shape::Rock) => 0,
            (Shape::Scissors, Shape::Paper) => 6,
            (Shape::Scissors, Shape::Scissors) => 3,
        };

        shape_score + outcome_score
    }
}

#[derive(Copy, Clone, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = UnknownInput;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => unreachable!(),
        }
    }
}

fn parse_input(input: &str) {}

fn part1(input: &str) -> u64 {
    let mut total = 0;

    for line in input.lines() {
        let split: Vec<_> = line.split(' ').map(|s| s.parse().unwrap()).collect();
        let round = Round {
            my_shape: split[1],
            opp_shape: split[0],
        };

        total += round.total_score();
    }

    total
}

// fn part2(input: &str) -> u64 {
//     let elves = parse_input(input);
//     let mut calories: Vec<u64> = elves.iter().map(Elf::total_calories).collect();
//     calories.sort();

//     calories.iter().rev().take(3).sum()
// }

// fn main() {
//     println!("part1: {}", part1(INPUT));
//     println!("part2: {}", part2(INPUT));
// }

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day02_sample.txt");

    #[test]
    fn test_day01_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 15)
    }

    #[test]
    fn test_day01() {
        assert_eq!(part1(INPUT), 12276)
    }

    // #[test]
    // fn test_day02_sample() {
    //     assert_eq!(part2(SAMPLE_INPUT), 45000)
    // }

    // #[test]
    // fn test_day02() {
    //     assert_eq!(part2(INPUT), 205381)
    // }
}
