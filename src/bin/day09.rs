use std::collections::HashSet;
use std::convert::Infallible;
use std::str::FromStr;

const INPUT: &'static str = include_str!("../../inputs/day09.txt");

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Step {
    direction: Direction,
    distance: u64,
}

impl FromStr for Step {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = s.split(' ').collect();
        let direction = match split[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!(),
        };

        let distance = u64::from_str(split[1]).unwrap();

        Ok(Self {
            direction,
            distance,
        })
    }
}

fn part1(input: &str) -> u64 {
    let steps: Vec<_> = input
        .lines()
        .map(|line| Step::from_str(line).unwrap())
        .collect();

    let mut vists: HashSet<(i64, i64)> = HashSet::new();

    let mut head = (0, 0);
    let mut tail = (0, 0);

    vists.insert(tail);

    for step in steps {
        for _ in 0..step.distance {
            let prev_head = head;

            match step.direction {
                Direction::Up => head.1 += 1,
                Direction::Down => head.1 -= 1,
                Direction::Left => head.0 -= 1,
                Direction::Right => head.0 += 1,
            }

            if (head.1 - tail.1).abs() > 1 || (head.0 - tail.0).abs() > 1 {
                tail = prev_head;
                vists.insert(tail);
            }
        }
    }

    vists.len() as u64
}

fn part2(input: &str) -> u64 {
    let steps: Vec<_> = input
        .lines()
        .map(Step::from_str)
        .map(Result::unwrap)
        .collect();

    let mut vists: HashSet<(i64, i64)> = HashSet::new();
    let mut knots = [(0i64, 0i64); 10];

    vists.insert(knots[9]);

    for step in steps {
        for _ in 0..step.distance {
            match step.direction {
                Direction::Up => knots[0].1 += 1,
                Direction::Down => knots[0].1 -= 1,
                Direction::Left => knots[0].0 -= 1,
                Direction::Right => knots[0].0 += 1,
            }

            for i in 1..10 {
                if (knots[i - 1].1 - knots[i].1).abs() > 1
                    || (knots[i - 1].0 - knots[i].0).abs() > 1
                {
                    knots[i] = *vec![
                        (knots[i].0, knots[i].1 + 1),     // N
                        (knots[i].0 + 1, knots[i].1 + 1), // NE
                        (knots[i].0 + 1, knots[i].1),     // E
                        (knots[i].0 + 1, knots[i].1 - 1), // SE
                        (knots[i].0, knots[i].1 - 1),     // S
                        (knots[i].0 - 1, knots[i].1 - 1), // SW
                        (knots[i].0 - 1, knots[i].1),     // W
                        (knots[i].0 - 1, knots[i].1 + 1), // NW
                    ]
                    .iter()
                    .min_by_key(|loc| {
                        (loc.1 - knots[i - 1].1).abs() + (loc.0 - knots[i - 1].0).abs()
                    })
                    .unwrap()
                }
            }

            vists.insert(knots[9]);
        }
    }

    vists.len() as u64
}

fn main() {
    println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day09_sample.txt");
    const SAMPLE_INPUT2: &'static str = include_str!("../../inputs/day09_sample2.txt");

    #[test]
    fn test_day01_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 13);
    }

    #[test]
    fn test_day01() {
        assert_eq!(part1(INPUT), 5874);
    }

    #[test]
    fn test_day02_sample() {
        assert_eq!(part2(SAMPLE_INPUT2), 36);
    }

    #[test]
    fn test_day02() {
        assert_eq!(part2(INPUT), 2467);
    }
}