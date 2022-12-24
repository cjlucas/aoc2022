use std::collections::{HashMap, VecDeque};

const INPUT: &'static str = include_str!("../../inputs/day12.txt");

fn find_shortest_path(map: &HashMap<(i64, i64), char>, start: (i64, i64)) -> Option<u64> {
    let mut explored: HashMap<(i64, i64), (i64, i64)> = HashMap::new();

    let mut q = VecDeque::<(i64, i64)>::new();

    explored.insert(start, start);
    q.push_back(start);

    while let Some(point) = q.pop_front() {
        if map[&point] == 'E' {
            let mut answer = 1;
            let mut cur = point;
            loop {
                let parent = explored[&cur];
                if parent == start {
                    return Some(answer);
                }

                cur = parent;
                answer += 1;
            }
        }

        let edges: Vec<(i64, i64)> = vec![
            (point.0, point.1 - 1), //N
            (point.0 + 1, point.1), //E
            (point.0, point.1 + 1), //S
            (point.0 - 1, point.1), //W
        ]
        .into_iter()
        .filter(|p| !explored.contains_key(&p))
        .filter(|p| map.contains_key(&p))
        .filter(|p| {
            let src = if map[&point] == 'S' { 'a' } else { map[&point] };
            let dest = if map[&p] == 'E' { 'z' } else { map[&p] };

            dest as i64 - src as i64 <= 1
        })
        .collect();

        for edge in edges {
            explored.insert(edge, point);
            q.push_back(edge);
        }
    }

    None
}

fn part1(input: &str) -> u64 {
    let mut map: HashMap<(i64, i64), char> = HashMap::new();

    let mut start = None;
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let p = (x as i64, y as i64);

            if ch == 'S' {
                start = Some(p);
            }

            map.insert(p, ch);
        }
    }

    find_shortest_path(&map, start.unwrap()).unwrap()
}

fn part2(input: &str) -> u64 {
    let mut map: HashMap<(i64, i64), char> = HashMap::new();

    let mut candidates = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let p = (x as i64, y as i64);

            if ch == 'S' || ch == 'a' {
                candidates.push(p);
            }

            map.insert(p, ch);
        }
    }

    candidates
        .iter()
        .filter_map(|start| find_shortest_path(&map, *start))
        .min()
        .unwrap()
}

fn main() {
    println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day12_sample.txt");

    #[test]
    fn test_day12_part1_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 31);
    }

    #[test]
    fn test_day12_part1() {
        assert_eq!(part1(INPUT), 394);
    }

    #[test]
    fn test_day12_part2_sample() {
        assert_eq!(part2(SAMPLE_INPUT), 29);
    }

    #[test]
    fn test_day12_part2() {
        assert_eq!(part2(INPUT), 388);
    }
}
