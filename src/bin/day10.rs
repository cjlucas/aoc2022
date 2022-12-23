use std::str::FromStr;

const INPUT: &'static str = include_str!("../../inputs/day10.txt");

#[derive(Debug, Clone)]
struct CPU {
    x: i64,
}

impl CPU {
    fn process_instruction(&self, instruction: &Instruction) -> Vec<Self> {
        use Instruction::*;

        match instruction {
            AddX(i) => vec![self.clone(), Self { x: self.x + i }],
            Noop => vec![self.clone()],
        }
    }
}

#[derive(Debug)]
enum Instruction {
    AddX(i64),
    Noop,
}

impl FromStr for Instruction {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = s.split(' ').collect();

        let inst = match split[0] {
            "addx" => Instruction::AddX(i64::from_str(split[1]).unwrap()),
            "noop" => Instruction::Noop,
            _ => unreachable!(),
        };

        Ok(inst)
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(Instruction::from_str)
        .map(Result::unwrap)
        .collect()
}

fn part1(input: &str) -> i64 {
    let instructions = parse_input(input);

    let mut cycles = vec![CPU { x: 1 }];
    for inst in instructions {
        for cycle in cycles.iter().last().unwrap().process_instruction(&inst) {
            cycles.push(cycle);
        }
    }

    vec![20, 60, 100, 140, 180, 220]
        .iter()
        .map(|cycle| *cycle as i64 * cycles[cycle - 1].x)
        .sum()
}

fn part2(input: &str) -> String {
    let instructions = parse_input(input);
    let mut cycles = vec![CPU { x: 1 }];
    for inst in instructions {
        for cycle in cycles.iter().last().unwrap().process_instruction(&inst) {
            cycles.push(cycle);
        }
    }

    let mut answer = String::new();

    for cycle in 1..=240 {
        let x = cycles[cycle - 1].x;
        let pixel = (cycle as i64 - 1) % 40;

        if (x - 1..=x + 1).contains(&pixel) {
            answer.push('#');
        } else {
            answer.push('.');
        }

        if cycle % 40 == 0 {
            answer.push('\n');
        }
    }

    answer
}

fn main() {
    println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day10_sample.txt");

    #[test]
    fn test_day01_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 13140);
    }

    #[test]
    fn test_day01() {
        assert_eq!(part1(INPUT), 13920);
    }

    #[test]
    fn test_day02_sample() {
        const EXPECTED_RESULT: &'static str =
            include_str!("../../inputs/day10_part2_sample_answer.txt");
        assert_eq!(part2(SAMPLE_INPUT), EXPECTED_RESULT);
    }

    #[test]
    fn test_day02() {
        const EXPECTED_RESULT: &'static str = include_str!("../../inputs/day10_part2_answer.txt");
        assert_eq!(part2(INPUT), EXPECTED_RESULT);
    }
}
