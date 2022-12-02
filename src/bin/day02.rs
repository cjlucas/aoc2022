use std::str::FromStr;

const INPUT: &'static str = include_str!("../../inputs/day02.txt");

#[derive(Debug)]
struct UnknownInput;

#[derive(Debug)]
struct Round {
    my_shape: Shape,
    opp_shape: Shape,
}

#[derive(Debug)]
enum RoundResult {
    Win,
    Lose,
    Draw,
}

impl RoundResult {
    fn score(&self) -> u64 {
        match &self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
}

impl FromStr for RoundResult {
    type Err = UnknownInput;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => unreachable!(),
        }
    }
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

impl Shape {
    fn score(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn winning_shape(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn losing_shape(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
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

fn part2(input: &str) -> u64 {
    let mut total = 0;

    for line in input.lines() {
        let split: Vec<_> = line.split(' ').collect();
        let opp_shape = split[0].parse::<Shape>().unwrap();
        let result = split[1].parse::<RoundResult>().unwrap();

        let my_shape = match &result {
            RoundResult::Lose => opp_shape.losing_shape(),
            RoundResult::Draw => opp_shape,
            RoundResult::Win => opp_shape.winning_shape(),
        };

        total += Round {
            my_shape,
            opp_shape,
        }
        .total_score();
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

    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day02_sample.txt");

    #[test]
    fn test_day01_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 15)
    }

    #[test]
    fn test_day01() {
        assert_eq!(part1(INPUT), 12276)
    }

    #[test]
    fn test_day02_sample() {
        assert_eq!(part2(SAMPLE_INPUT), 12)
    }

    #[test]
    fn test_day02() {
        assert_eq!(part2(INPUT), 9975)
    }
}
