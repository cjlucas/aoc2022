const INPUT: &'static str = include_str!("../../inputs/day01.txt");

#[derive(Default, Debug)]
struct Elf {
    calories: Vec<u64>,
}

impl Elf {
    fn total_calories(&self) -> u64 {
        self.calories.iter().sum()
    }
}

fn parse_input(input: &str) -> Vec<Elf> {
    let mut elves = vec![];

    let mut elf = Elf::default();

    for line in input.lines() {
        if line.len() == 0 {
            elves.push(elf);
            elf = Elf::default();
            continue;
        }

        elf.calories.push(line.parse().unwrap())
    }

    return elves;
}

fn part1(input: &str) -> u64 {
    let elves = parse_input(input);
    elves.iter().map(|elf| elf.total_calories()).max().unwrap()
}

fn main() {}

#[test]
fn test_day01_sample() {
    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day01_sample.txt");

    assert_eq!(part1(SAMPLE_INPUT), 24000)
}

#[test]
fn test_day01() {
    assert_eq!(part1(INPUT), 70296)
}
