#![allow(unused)]

use advent_2023::lcm::lcm;
use anyhow::{bail, Error, Result};
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
    sync::Arc,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => bail!("Unexpected input"),
        }
    }
}

impl Direction {
    fn from_input<'a>(line: &mut impl Iterator<Item = &'a str>) -> Vec<Direction> {
        line.next()
            .map(|line| {
                line.trim()
                    .chars()
                    .filter_map(|ch| ch.try_into().ok())
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    value: [u8; 3],
    left: [u8; 3],
    right: [u8; 3],
}

impl Node {
    fn follow<'a>(
        &self,
        dir: &Direction,
        dictionary: &'a HashMap<[u8; 3], Arc<Node>>,
    ) -> Option<&'a Arc<Node>> {
        match dir {
            Direction::Left => dictionary.get(&self.left),
            Direction::Right => dictionary.get(&self.right),
        }
    }

    fn is_match_value(&self, scan: &[u8; 3]) -> bool {
        for (i, end_ch) in scan.iter().enumerate() {
            if end_ch == &b'.' {
                continue;
            }
            if end_ch != &self.value[i] {
                return false;
            }
        }
        true
    }

    fn extract_from_line<'a>(line: &mut impl Iterator<Item = &'a str>) -> Option<([u8; 3], Node)> {
        line.next()
            .and_then(|line| {
                let mut split = line.trim().split('=');

                let source: Option<[u8; 3]> = split
                    .next()
                    .and_then(|s| s.trim().as_bytes().try_into().ok());
                let connections: Option<[[u8; 3]; 2]> = split
                    .next()
                    .map(|s| {
                        s.chars()
                            .skip_while(|ch| !ch.is_ascii_alphanumeric())
                            .take_while(|ch| ch != &')')
                            .collect::<String>()
                            .split(", ")
                            .filter_map(|s| s.as_bytes().try_into().ok())
                            .collect::<Vec<[u8; 3]>>()
                    })
                    .and_then(|b| b.try_into().ok());
                source.zip(connections)
            })
            .map(|(source, [left, right])| {
                (
                    source,
                    Node {
                        value: source,
                        left,
                        right,
                    },
                )
            })
    }
}

#[derive(Debug, Clone)]
struct Map {
    dictionary: HashMap<[u8; 3], Arc<Node>>,
}

impl Map {
    fn from_input<'a>(line: &mut impl Iterator<Item = &'a str>) -> Map {
        let mut dictionary = HashMap::new();
        while let Some((code, node)) = Node::extract_from_line(line) {
            dictionary.insert(code, Arc::new(node));
        }
        Map { dictionary }
    }

    fn steps_to_exit(&self, path: &[Direction], start: [u8; 3], end: [u8; 3]) -> usize {
        self.dictionary
            .par_iter()
            .filter(|(k, _)| {
                for i in 0..3 {
                    if start[i] == b'.' {
                        continue;
                    }
                    if start[i] != k[i] {
                        return false;
                    }
                }
                true
            })
            .map(|(_k, v)| v)
            .map(|start_node| {
                let mut seen = HashSet::new();
                let mut loop_steps: usize = 0;
                let mut path_i = 0;
                let mut node = start_node;
                while !seen.contains(&(path_i, node.value)) {
                    seen.insert((path_i, node.value));
                    loop_steps += 1;
                    node = node
                        .follow(&path[path_i], &self.dictionary)
                        .expect("to have a path");

                    if node.is_match_value(&end) {
                        return loop_steps;
                    }
                    path_i += 1;
                    if path_i == path.len() {
                        path_i = 0;
                    }
                }
                panic!("Match not found")
            })
            .reduce(|| 1, lcm)
    }
}

pub fn min_steps(input: &str, start: &str, end: &str) -> usize {
    let mut lines = input.trim().split('\n');
    let directions = Direction::from_input(&mut lines);
    lines.next();
    let map = Map::from_input(&mut lines);
    map.steps_to_exit(
        &directions,
        start.as_bytes().try_into().unwrap(),
        end.as_bytes().try_into().unwrap(),
    )
}

fn main() {
    let input = include_str!("../input/day8_haunted_wasteland.txt");

    let start = std::time::Instant::now();
    println!("{}", min_steps(input, "AAA", "ZZZ"));
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);

    let start = std::time::Instant::now();
    println!("{}", min_steps(input, "..A", "..Z"));
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_steps_example_1() {
        let input = r#"
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        
        "#;
        assert_eq!(min_steps(input, "AAA", "ZZZ"), 2);
    }

    #[test]
    fn test_min_steps_example_2() {
        let input = r#"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "#;
        assert_eq!(min_steps(input, "AAA", "ZZZ"), 6);
    }
    #[test]
    fn test_min_steps_example_3() {
        let input = r#"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "#;
        assert_eq!(min_steps(input, "..A", "..Z"), 6);
    }
}
