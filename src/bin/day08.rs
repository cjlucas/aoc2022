use std::str::FromStr;

const INPUT: &'static str = include_str!("../../inputs/day08.txt");

#[derive(Debug)]
struct Forest {
    rows: Vec<Vec<Tree>>,
}

impl Forest {
    fn new() -> Self {
        Forest { rows: vec![] }
    }

    fn add_row(&mut self, row: Vec<Tree>) {
        self.rows.push(row)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Tree {
    height: u64,
}

fn parse_input(input: &str) -> Vec<Vec<Tree>> {
    let mut forest = vec![];

    for line in input.lines() {
        let row = line
            .chars()
            .map(|c| Tree {
                height: u64::from_str(&c.to_string()).unwrap(),
            })
            .collect();

        forest.push(row);
    }

    forest
}

fn is_visible(trees: &Vec<Vec<Tree>>, row: usize, col: usize) -> bool {
    if row == 0 || col == 0 || row == trees.len() - 1 || col == trees[row].len() - 1 {
        return true;
    }

    let tree = &trees[row][col];

    if (0..row).map(|i| &trees[i][col]).all(|t| t < tree) {
        return true;
    }

    if (row + 1..trees.len())
        .map(|i| &trees[i][col])
        .all(|t| t < tree)
    {
        return true;
    }

    if (0..col).map(|i| &trees[row][i]).all(|t| t < tree) {
        return true;
    }

    if (col + 1..trees[row].len())
        .map(|i| &trees[row][i])
        .all(|t| t < tree)
    {
        return true;
    }

    false
}

fn part1(input: &str) -> u64 {
    let trees = parse_input(input);

    let mut answer = 0;

    for row in 0..trees.len() {
        assert!(trees[row].len() == trees[0].len());
        for col in 0..trees[row].len() {
            if is_visible(&trees, row, col) {
                answer += 1;
            }
        }
    }

    answer
}

fn part2(input: &str) -> u64 {
    let trees = parse_input(input);

    let mut answer = 0;

    for row in 0..trees.len() {
        for col in 0..trees[row].len() {
            let tree = &trees[row][col];
            let mut viewing_distances = vec![0u64, 0, 0, 0];

            for i in (0..row).rev() {
                viewing_distances[0] += 1;
                if &trees[i][col] >= tree {
                    break;
                }
            }

            for i in (row + 1..trees.len()) {
                viewing_distances[1] += 1;
                if &trees[i][col] >= tree {
                    break;
                }
            }

            for i in (0..col).rev() {
                viewing_distances[2] += 1;
                if &trees[row][i] >= tree {
                    break;
                }
            }

            for i in (col + 1..trees[row].len()) {
                viewing_distances[3] += 1;
                if &trees[row][i] >= tree {
                    break;
                }
            }

            let scenic_score = viewing_distances.iter().fold(1, |acc, x| acc * x);

            // println!(
            //     "({}, {}) = {:?} = {}",
            //     row, col, &viewing_distances, scenic_score
            // );

            if scenic_score > answer {
                answer = scenic_score;
            }
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

    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day08_sample.txt");

    #[test]
    fn test_day01_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 21);
    }

    #[test]
    fn test_day01() {
        assert_eq!(part1(INPUT), 1705);
    }

    #[test]
    fn test_day02_sample() {
        assert_eq!(part2(SAMPLE_INPUT), 8);
    }

    #[test]
    fn test_day02() {
        assert_eq!(part2(INPUT), 371200);
    }
}
