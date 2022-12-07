use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

const INPUT: &'static str = include_str!("../../inputs/day05.txt");

#[derive(Debug)]
struct Stack {
    crates: VecDeque<char>,
}

fn part1(input: &str) -> u64 {
    let mut map: HashMap<usize, Vec<&str>> = HashMap::new();

    let mut lines_iter = input.lines();

    for line in &mut lines_iter {
        if line.len() == 0 {
            break;
        }
        let v: Vec<_> = line.match_indices(char::is_uppercase).collect();

        for f in v {
            let key = f.0 / 4;

            if let None = map.get(&key) {
                map.insert(key, vec![]);
            }

            map.get_mut(&key).unwrap().push(f.1);
        }
    }

    let mut stacks: Vec<Stack> = vec![];

    for stack_num in 0..map.len() {
        let crates = map
            .get(&stack_num)
            .unwrap()
            .iter()
            .map(|c| char::from_str(c).unwrap())
            .collect();

        stacks.push(Stack { crates });
    }

    dbg!(&stacks);

    for line in &mut lines_iter {
        dbg!(&line);
        let inst: Vec<_> = line
            .matches(char::is_numeric)
            .map(|c| u64::from_str(c).unwrap())
            .collect();

        dbg!(&inst);
        let num_moves = inst[0];
        let mut src = stacks.get_mut(inst[1] as usize - 1).unwrap();

        let mut temp: Vec<char> = vec![];

        for _ in 0..num_moves {
            let c = src.crates.pop_front().unwrap();
            temp.push(c);
        }

        let mut dest = stacks.get_mut(inst[2] as usize - 1).unwrap();

        for c in temp {
            dest.crates.push_front(c);
        }

        dbg!(&stacks);
    }

    let mut answer: String = stacks
        .iter()
        .map(|stack| stack.crates.front().unwrap_or(&' '))
        .collect();

    dbg!(answer);
    0
}

fn part2(input: &str) -> u64 {
    0
}

fn main() {
    println!("part1: {}", part1(INPUT));
    // println!("part2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day05_sample.txt");

    #[test]
    fn test_day01_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 157);
    }

    // #[test]
    // fn test_day01() {
    //     assert_eq!(part1(INPUT), 7766);
    // }

    // #[test]
    // fn test_day02_sample() {
    //     assert_eq!(part2(SAMPLE_INPUT), 70);
    // }

    // #[test]
    // fn test_day02() {
    //     assert_eq!(part2(INPUT), 2415);
    // }
}
