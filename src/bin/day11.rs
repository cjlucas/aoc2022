use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

fn input() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![89, 84, 88, 78, 70].into_iter().collect(),
            operation: WorryOp::Mul(5),
            test: WorryTest::Div(7),
            destinations: Destinations { t: 6, f: 7 },
        },
        Monkey {
            items: vec![76, 62, 61, 54, 69, 60, 85].into_iter().collect(),
            operation: WorryOp::Add(1),
            test: WorryTest::Div(17),
            destinations: Destinations { t: 0, f: 6 },
        },
        Monkey {
            items: vec![83, 89, 53].into_iter().collect(),
            operation: WorryOp::Add(8),
            test: WorryTest::Div(11),
            destinations: Destinations { t: 5, f: 3 },
        },
        Monkey {
            items: vec![95, 94, 85, 57].into_iter().collect(),
            operation: WorryOp::Add(4),
            test: WorryTest::Div(13),
            destinations: Destinations { t: 0, f: 1 },
        },
        Monkey {
            items: vec![82, 98].into_iter().collect(),
            operation: WorryOp::Add(7),
            test: WorryTest::Div(19),
            destinations: Destinations { t: 5, f: 2 },
        },
        Monkey {
            items: vec![69].into_iter().collect(),
            operation: WorryOp::Add(2),
            test: WorryTest::Div(2),
            destinations: Destinations { t: 1, f: 3 },
        },
        Monkey {
            items: vec![82, 70, 58, 87, 59, 99, 92, 65].into_iter().collect(),
            operation: WorryOp::Mul(11),
            test: WorryTest::Div(5),
            destinations: Destinations { t: 7, f: 4 },
        },
        Monkey {
            items: vec![91, 53, 96, 98, 68, 82].into_iter().collect(),
            operation: WorryOp::Sqr,
            test: WorryTest::Div(3),
            destinations: Destinations { t: 4, f: 2 },
        },
    ]
}

#[derive(Clone, Debug)]
enum WorryOp {
    Mul(u64),
    Add(u64),
    Sqr,
}

#[derive(Clone, Debug)]
enum WorryTest {
    Div(u64),
}

#[derive(Clone, Debug)]
struct Destinations {
    t: usize,
    f: usize,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: WorryOp,
    test: WorryTest,
    destinations: Destinations,
}

impl Monkey {
    fn test_worry_level(&self, worry_level: u64) -> bool {
        match self.test {
            WorryTest::Div(i) => worry_level % i == 0,
        }
    }
}

fn part1(mut input: Vec<Monkey>) -> u64 {
    let mut items_inspected = vec![0; input.len()];
    let monkeys: Vec<RefCell<Monkey>> = input.into_iter().map(RefCell::new).collect();

    for _ in 0..20 {
        for (idx, ref_cell) in monkeys.iter().enumerate() {
            let mut monkey = ref_cell.borrow_mut();

            while let Some(item) = monkey.items.pop_front() {
                items_inspected[idx] += 1;

                // dbg!(&item);
                let worry_level = match monkey.operation {
                    WorryOp::Add(i) => item + i,
                    WorryOp::Mul(i) => item * i,
                    WorryOp::Sqr => item * item,
                };
                // dbg!(&worry_level);

                let worry_level = worry_level / 3;
                // dbg!(&worry_level);

                let dest = if monkey.test_worry_level(worry_level) {
                    monkey.destinations.t
                } else {
                    monkey.destinations.f
                };

                // println!("moving {} from {} to {}", worry_level, idx, dest);

                monkeys[dest].borrow_mut().items.push_back(worry_level);
            }
        }
    }

    // dbg!(&monkeys);

    items_inspected.sort();
    items_inspected
        .iter()
        .rev()
        .take(2)
        .fold(1, |acc, x| acc * x)
}

fn part2(input: Vec<Monkey>) -> u64 {
    let mut items_inspected = vec![0; input.len()];
    let monkeys: Vec<RefCell<Monkey>> = input.into_iter().map(RefCell::new).collect();

    for round in 0..10_000 {
        for (idx, ref_cell) in monkeys.iter().enumerate() {
            let mut monkey = ref_cell.borrow_mut();

            while let Some(item) = monkey.items.pop_front() {
                items_inspected[idx] += 1;

                // println!("{} {:?}", item, monkey.operation);
                let worry_level = match monkey.operation {
                    WorryOp::Add(i) => item + i,
                    WorryOp::Mul(i) => item * i,
                    WorryOp::Sqr => item * item,
                };
                // dbg!(&worry_level);

                // dbg!(&worry_level);

                let (worry_level, dest) = if monkey.test_worry_level(worry_level) {
                    let i = match monkey.test {
                        WorryTest::Div(i) => i,
                    };

                    (i, monkey.destinations.t)
                } else {
                    let i = match monkey.test {
                        WorryTest::Div(i) => i,
                    };

                    (i + (worry_level % i), monkey.destinations.f)
                };

                // println!("moving {} from {} to {}", worry_level, idx, dest);

                monkeys[dest].borrow_mut().items.push_back(worry_level);
            }
        }
    }

    items_inspected.sort();
    items_inspected
        .iter()
        .rev()
        .take(2)
        .fold(1, |acc, x| acc * x)
}

fn main() {
    println!("part1: {}", part1(input()));
    // println!("part2: {}", part2(input()));
}

#[cfg(test)]
mod tests {
    use super::*;

    // const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day10_sample.txt");

    fn sample_input() -> Vec<Monkey> {
        vec![
            Monkey {
                items: vec![79, 98].into_iter().collect(),
                operation: WorryOp::Mul(19),
                test: WorryTest::Div(23),
                destinations: Destinations { t: 2, f: 3 },
            },
            Monkey {
                items: vec![54, 65, 75, 74].into_iter().collect(),
                operation: WorryOp::Add(6),
                test: WorryTest::Div(19),
                destinations: Destinations { t: 2, f: 0 },
            },
            Monkey {
                items: vec![79, 60, 97].into_iter().collect(),
                operation: WorryOp::Sqr,
                test: WorryTest::Div(13),
                destinations: Destinations { t: 1, f: 3 },
            },
            Monkey {
                items: vec![74].into_iter().collect(),
                operation: WorryOp::Add(3),
                test: WorryTest::Div(17),
                destinations: Destinations { t: 0, f: 1 },
            },
        ]
    }

    #[test]
    fn test_day01_sample() {
        assert_eq!(part1(sample_input()), 10605);
    }

    #[test]
    fn test_day01() {
        assert_eq!(part1(input()), 55930);
    }

    #[test]
    fn test_day02_sample() {
        assert_eq!(part2(sample_input()), 2713310158);
    }

    // #[test]
    // fn test_day02() {
    //     const EXPECTED_RESULT: &'static str = include_str!("../../inputs/day10_part2_answer.txt");
    //     assert_eq!(part2(INPUT), EXPECTED_RESULT);
    // }
}
