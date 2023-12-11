use anyhow::{bail, Result};
use std::{
    collections::{HashSet, VecDeque},
    fmt::{Display, Write},
};

#[derive(Debug, Clone, Copy)]
struct Galaxy {
    col: usize,
    row: usize,
}

impl Galaxy {
    fn new(col: usize, row: usize) -> Galaxy {
        Galaxy { col, row }
    }
}

#[derive(Debug)]
struct StarMap {
    map: Vec<Vec<char>>,
    galaxies: Vec<Galaxy>,
    empty_rows: HashSet<usize>,
    empty_cols: HashSet<usize>,
}

impl StarMap {
    fn pair_galaxies(&self) -> Vec<(Galaxy, Galaxy)> {
        let mut res = vec![];
        for i in 0..self.galaxies.len() {
            for j in i + 1..self.galaxies.len() {
                res.push((self.galaxies[i], self.galaxies[j]))
            }
        }
        res
    }

    fn emptiness_between_cols(&self, first: usize, second: usize, expansion: usize) -> usize {
        let mut max = first;
        let mut min = second;
        if min > max {
            std::mem::swap(&mut max, &mut min);
        }
        let count = self
            .empty_cols
            .iter()
            .filter(|&i| i > &min && i < &max)
            .count();
        max - min - count + count * expansion
    }

    fn emptiness_between_rows(&self, first: usize, second: usize, expansion: usize) -> usize {
        let mut max = first;
        let mut min = second;
        if min > max {
            std::mem::swap(&mut max, &mut min);
        }
        let count = self
            .empty_rows
            .iter()
            .filter(|&i| i > &min && i < &max)
            .count();
        max - min - count + count * expansion
    }

    fn distance(&self, left: &Galaxy, right: &Galaxy, expansion: usize) -> usize {
        self.emptiness_between_cols(left.col, right.col, expansion)
            + self.emptiness_between_rows(left.row, right.row, expansion)
    }

    fn from_str(s: &str) -> Result<Self> {
        let map: Vec<Vec<char>> = s
            .trim()
            .split('\n')
            .map(|l| l.trim().chars().collect())
            .filter(|r: &Vec<char>| !r.is_empty())
            .collect();
        if map.is_empty() {
            bail!("Map cannot be empty")
        }
        let cols_count = map.first().unwrap().len();
        if map.iter().any(|r| r.len() != cols_count) {
            bail!("Map contains not equal rows")
        }
        let empty_rows: HashSet<usize> = map
            .iter()
            .enumerate()
            .filter(|(_, row)| row.iter().all(|ch| ch == &'.'))
            .map(|(i, _)| i)
            .collect();
        let empty_cols: HashSet<usize> = (0..cols_count)
            .filter(|i| map.iter().all(|r| r[*i] == '.'))
            .collect();

        let galaxies: Vec<Galaxy> = map
            .iter()
            .enumerate()
            .flat_map(|(row_i, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(col_i, &ch)| (row_i, col_i, ch))
            })
            .filter(|(_, _, ch)| ch == &'#')
            .map(|(row, col, _)| Galaxy::new(col, row))
            .collect();
        Ok(Self {
            map,
            galaxies,
            empty_cols,
            empty_rows,
        })
    }
}

impl Display for StarMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(' ')?;
        for i in 0..self.map.first().unwrap().len() {
            if self.empty_cols.contains(&i) {
                f.write_char('v')?;
            } else {
                f.write_char(' ')?;
            }
        }
        f.write_char('\n')?;
        let mut start_count = 0;
        for (i, row) in self.map.iter().enumerate() {
            if self.empty_rows.contains(&i) {
                f.write_char('>')?;
            } else {
                f.write_char(' ')?;
            }
            let mut star_num = VecDeque::new();
            for ch in row.iter() {
                if ch == &'#' {
                    start_count += 1;
                    star_num.clear();
                    star_num.extend(start_count.to_string().chars());
                }
                if let Some(ch) = star_num.pop_front() {
                    f.write_char(ch)?;
                } else {
                    f.write_char(*ch)?;
                }
            }
            if self.empty_rows.contains(&i) {
                f.write_char('<')?;
            }
            f.write_char('\n')?;
        }
        f.write_char(' ')?;
        for i in 0..self.map.first().unwrap().len() {
            if self.empty_cols.contains(&i) {
                f.write_char('^')?;
            } else {
                f.write_char(' ')?;
            }
        }
        f.write_char('\n')?;
        f.write_fmt(format_args!(
            "cols {:?} rows {:?}",
            self.empty_cols, self.empty_rows
        ))?;
        f.write_char('\n')?;
        for (i, g) in self.galaxies.iter().enumerate() {
            f.write_fmt(format_args!("{:3} -> ({:3}, {:3})\n", i + 1, g.col, g.row))?;
        }
        Ok(())
    }
}

fn sum_closest_pairs(input: &str, expansions: Vec<usize>) -> Result<Vec<usize>> {
    let map: StarMap = StarMap::from_str(input)?;
    println!("{map}");
    let galaxy_pairs = map.pair_galaxies();

    Ok(expansions
        .into_iter()
        .map(|expansion| {
            galaxy_pairs
                .iter()
                .map(|(left, right)| map.distance(left, right, expansion))
                .sum()
        })
        .collect())
}

fn main() -> Result<()> {
    let input = include_str!("../input/day11_cosmic_expansion.txt");

    let start = std::time::Instant::now();
    println!("{:?}", sum_closest_pairs(input, vec![2, 1_000_000])?);
    let duration = start.elapsed();

    println!("Time elapsed is: {duration:?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = r#"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "#;
        assert_eq!(
            sum_closest_pairs(input, vec![2, 10, 100]).unwrap(),
            vec![374, 1030, 8410]
        );
    }
}
