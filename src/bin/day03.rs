use std::collections::HashSet;
use std::convert::Infallible;
use std::str::FromStr;

const INPUT: &'static str = include_str!("../../inputs/day03.txt");

struct Rucksack {
    items: Vec<Item>,
}

impl FromStr for Rucksack {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rucksack {
            items: s.chars().map(Item::from).collect(),
        })
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Item(char);

impl Item {
    fn priority(&self) -> u64 {
        let foo = if self.0.is_uppercase() { 38 } else { 96 };

        TryInto::<u32>::try_into(self.0).unwrap() as u64 - foo
    }
}

impl From<char> for Item {
    fn from(c: char) -> Self {
        Self(c)
    }
}

fn common_items<'a, T>(v: Vec<T>) -> HashSet<&'a Item>
where
    T: IntoIterator<Item = &'a Item> + Clone,
{
    let sets: Vec<_> = v.iter().cloned().map(|x| HashSet::from_iter(x)).collect();
    sets.iter()
        .cloned()
        .reduce(|acc, x| acc.intersection(&x).cloned().collect())
        .unwrap()
}

fn part1(input: &str) -> u64 {
    let mut total = 0;

    for line in input.lines() {
        let rucksack = Rucksack::from_str(line).unwrap();

        let div = rucksack.items.len() / 2;
        let first: HashSet<_> = rucksack.items.iter().take(div).collect();
        let second: HashSet<_> = rucksack.items.iter().skip(div).collect();

        let common_items: HashSet<_> = common_items(vec![first, second]);

        total += common_items
            .iter()
            .cloned()
            .map(|item| item.priority())
            .max()
            .unwrap();
    }

    total
}

fn part2(input: &str) -> u64 {
    let mut total = 0;

    let lines: Vec<_> = input.lines().collect();

    for group in lines.chunks(3) {
        let first = Rucksack::from_str(group[0]).unwrap().items;
        let second = Rucksack::from_str(group[1]).unwrap().items;
        let third = Rucksack::from_str(group[2]).unwrap().items;

        let common_items: HashSet<_> = common_items(vec![&first, &second, &third]);

        total += common_items
            .iter()
            .map(|item| item.priority())
            .max()
            .unwrap();
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

    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day03_sample.txt");

    #[test]
    fn test_day01_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 157);
    }

    #[test]
    fn test_day01() {
        assert_eq!(part1(INPUT), 7766);
    }

    #[test]
    fn test_day02_sample() {
        assert_eq!(part2(SAMPLE_INPUT), 70);
    }

    #[test]
    fn test_day02() {
        assert_eq!(part2(INPUT), 2415);
    }
}
