use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

const INPUT: &'static str = include_str!("../../inputs/day05.txt");

#[derive(Debug)]
struct Stack {
    crates: VecDeque<char>,
}

fn parse_inst(input: &str) -> nom::IResult<&str, (usize, usize, usize)> {
    use nom::bytes::complete::tag;
    use nom::bytes::complete::take_while;
    use nom::combinator::map_res;

    let (input, _) = tag("move ")(input)?;
    let (input, num_moves) = map_res(take_while(char::is_numeric), usize::from_str)(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, src) = map_res(take_while(char::is_numeric), usize::from_str)(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, dest) = map_res(take_while(char::is_numeric), usize::from_str)(input)?;

    Ok((input, (num_moves, src, dest)))
}

fn part1(input: &str) -> String {
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
    dbg!(&stacks.len());

    for line in &mut lines_iter {
        // dbg!(&line);
        let (_, inst) = parse_inst(line).unwrap();

        dbg!(&inst);
        let num_moves = inst.0;
        let src = stacks.get_mut(inst.1 as usize - 1).unwrap();

        let mut temp: Vec<char> = vec![];

        for _ in 0..num_moves {
            // if let Some(c) = src.crates.pop_front() {
            //     temp.push(c);
            // }
            let c = src.crates.pop_front().unwrap();
            temp.push(c);
        }

        let dest = stacks.get_mut(inst.2 as usize - 1).unwrap();

        for c in temp {
            dest.crates.push_front(c);
        }

        // dbg!(&stacks);
    }

    let answer: String = stacks
        .iter()
        .map(|stack| stack.crates.front().unwrap_or(&' '))
        .collect();

    answer
}

fn part2(_input: &str) -> String {
    "".to_string()
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
        assert_eq!(part1(SAMPLE_INPUT), "CMZ");
    }

    #[test]
    fn test_parse_inst01() {
        assert_eq!(parse_inst("move 14 from 5 to 6"), Ok(("", (14, 5, 6))));
    }

    #[test]
    fn test_day01() {
        assert_eq!(part1(INPUT), "");
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
