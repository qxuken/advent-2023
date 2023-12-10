use anyhow::{anyhow, bail, Error, Result};
use std::{
    collections::HashSet,
    fmt::{Display, Write},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq)]
enum MazeMark {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    NoPipe,
    Start,
    Seen,
}

impl MazeMark {
    fn next_position(
        &self,
        (prev_x, prev_y): (usize, usize),
        (x, y): (usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            Self::NorthSouth => {
                if y > prev_y {
                    return Some((x, y + 1));
                }
                if y > 0 {
                    return Some((x, y - 1));
                }
            }
            Self::EastWest => {
                if x > prev_x {
                    return Some((x + 1, y));
                }
                if x > 0 {
                    return Some((x - 1, y));
                }
            }
            Self::NorthEast => {
                if x == prev_x {
                    return Some((x + 1, y));
                }
                if y > 0 {
                    return Some((x, y - 1));
                }
            }
            Self::NorthWest => {
                if y == prev_y && y > 0 {
                    return Some((x, y - 1));
                }
                if x == prev_x && x > 0 {
                    return Some((x - 1, y));
                }
            }
            Self::SouthWest => {
                if y == prev_y {
                    return Some((x, y + 1));
                }
                if x > 0 {
                    return Some((x - 1, y));
                }
            }
            Self::SouthEast => {
                if y == prev_y {
                    return Some((x, y + 1));
                }
                return Some((x + 1, y));
            }
            _ => return Some((x, y)),
        };
        None
    }

    fn can_visit(&self, (prev_x, prev_y): (usize, usize), (x, y): (usize, usize)) -> bool {
        match self {
            Self::NorthSouth => x == prev_x && (prev_y + 1 == y || (prev_y > 0 && prev_y - 1 == y)),
            Self::EastWest => y == prev_y && (prev_x + 1 == x || (prev_x > 0 && prev_x - 1 == x)),
            Self::NorthEast => {
                (y == prev_y && (prev_x > 0 && prev_x - 1 == x))
                    || ((prev_y > 0 && prev_y - 1 == y) && x == prev_x)
            }
            Self::NorthWest => {
                (y == prev_y && x == prev_x + 1) || ((prev_y > 0 && prev_y - 1 == y) && x == prev_x)
            }
            Self::SouthWest => {
                (y == prev_y && x == prev_x + 1) || ((prev_y > 0 && prev_y - 1 == y) && x == prev_x)
            }
            Self::SouthEast => {
                (y == prev_y && (prev_x > 0 && prev_x - 1 == x))
                    || ((prev_y > 0 && prev_y - 1 == y) && x == prev_x)
            }
            _ => false,
        }
    }
}

impl From<char> for MazeMark {
    fn from(ch: char) -> Self {
        match ch {
            '|' => Self::NorthSouth,
            '-' => Self::EastWest,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            'S' => Self::Start,
            '*' => Self::Seen,
            _ => Self::NoPipe,
        }
    }
}

impl Display for MazeMark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NorthSouth => f.write_char('|'),
            Self::EastWest => f.write_char('-'),
            Self::NorthEast => f.write_char('L'),
            Self::NorthWest => f.write_char('J'),
            Self::SouthWest => f.write_char('7'),
            Self::SouthEast => f.write_char('F'),
            Self::Start => f.write_char('S'),
            Self::Seen => f.write_char('*'),
            Self::NoPipe => f.write_char('.'),
        }
    }
}

#[derive(Debug)]
struct Maze {
    map: Vec<Vec<MazeMark>>,
}

impl FromStr for Maze {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let map = s
            .trim()
            .split('\n')
            .map(|s| s.trim().chars().map(MazeMark::from).collect())
            .filter(|r: &Vec<MazeMark>| !r.is_empty())
            .collect::<Vec<Vec<MazeMark>>>();
        if map.is_empty() {
            bail!("Map cannot be empty")
        }
        let length = map.first().unwrap().len();
        if map.iter().any(|r| r.len() != length) {
            bail!("Map contains not equal rows")
        }

        Ok(Self { map })
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("map")?;
        if let Some(r) = self.map.first() {
            if r.len() > 100 {
                f.write_str("\n   ")?;
                for (i, _) in r.iter().enumerate() {
                    f.write_char(' ')?;
                    if i < 100 {
                        f.write_char(' ')?;
                    } else {
                        f.write_str(&(i % 1000 / 100).to_string())?;
                    }
                }
            }
            if r.len() > 10 {
                f.write_str("\n   ")?;
                for (i, _) in r.iter().enumerate() {
                    f.write_char(' ')?;
                    if i < 10 {
                        f.write_char(' ')?;
                    } else {
                        f.write_str(&(i % 100 / 10).to_string())?;
                    }
                }
            }
            f.write_str("\n   ")?;
            for (i, _) in r.iter().enumerate() {
                f.write_char(' ')?;
                f.write_str(&(i % 10).to_string())?;
            }
        }
        f.write_str(" x\n")?;
        for (i, row) in self.map.iter().enumerate() {
            f.write_fmt(format_args!("{:3}", &i.to_string()))?;
            for mark in row.iter() {
                f.write_char(' ')?;
                f.write_str(&mark.to_string())?;
            }
            f.write_char('\n')?;
        }
        f.write_str("y\n")?;
        Ok(())
    }
}

impl Maze {
    fn get(&self, (x, y): (usize, usize)) -> Option<&MazeMark> {
        self.map.get(y).and_then(|r| r.get(x))
    }

    fn change(&mut self, (x, y): (usize, usize), val: MazeMark) -> Result<()> {
        let mark = self
            .map
            .get_mut(y)
            .and_then(|r| r.get_mut(x))
            .ok_or_else(|| anyhow!("Tried access undefined value"))?;
        *mark = val;
        Ok(())
    }
}

impl Maze {
    fn find_start(&self) -> Option<(usize, usize)> {
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if self.map[y][x] == MazeMark::Start {
                    return Some((y, y));
                }
            }
        }
        None
    }

    fn find_next_from_start(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        let mut scan_arr = vec![];
        if y > 0 {
            scan_arr.push((x, y - 1));
        }
        scan_arr.push((x + 1, y));
        scan_arr.push((x, y + 1));
        if x > 0 {
            scan_arr.push((x - 1, y));
        }
        scan_arr
            .into_iter()
            .find(|&pos| self.get(pos).is_some_and(|m| m.can_visit((x, y), pos)))
    }
}

impl Maze {
    fn walk_main_loop(&self) -> Result<usize> {
        let start = self
            .find_start()
            .ok_or_else(|| anyhow!("No starting point found"))?;
        // print!("0 - start {start:?}");

        let mut seen = HashSet::new();
        let mut path_len = 1;
        let mut prev_pos = start;
        seen.insert(start);
        let mut pos = self
            .find_next_from_start(start)
            .ok_or_else(|| anyhow!("No path from start"))?;

        while !seen.contains(&pos) {
            // print!("\n{path_len} - pos {pos:?}");
            seen.insert(pos);
            path_len += 1;
            if let Some(next_pos) = self
                .get(pos)
                .and_then(|mark| mark.next_position(prev_pos, pos))
            {
                // print!(" {mark}");
                prev_pos = pos;
                pos = next_pos;
            }
        }
        println!();
        Ok(path_len)
    }

    fn walk(&mut self, (x, y): (usize, usize)) -> Result<()> {
        let mark = self.get((x, y));
        if mark.is_none() || mark.is_some_and(|m| m != &MazeMark::NoPipe) {
            return Ok(());
        }
        self.change((x, y), MazeMark::Seen)?;
        for x in x.checked_sub(1).unwrap_or(x)..=x + 1 {
            for y in y.checked_sub(1).unwrap_or(y)..=y + 1 {
                self.walk((x, y))?;
            }
        }
        Ok(())
    }

    fn see_outside(&mut self) -> Result<()> {
        let y_len = self.map.len();
        let x_len = self
            .map
            .first()
            .ok_or_else(|| anyhow!("Map is empty"))?
            .len();
        for y in 0..y_len {
            self.walk((0, y))?;
            self.walk((x_len - 1, y))?;
        }

        for x in 1..x_len - 1 {
            self.walk((x, 0))?;
            self.walk((x, y_len - 1))?;
        }
        Ok(())
    }

    fn count_no_pipes(&self) -> usize {
        self.map
            .iter()
            .map(|r| r.iter().filter(|&m| m == &MazeMark::NoPipe).count())
            .sum()
    }
}

fn solve_maze(input: &str) -> Result<(usize, usize)> {
    let mut maze: Maze = input.parse()?;
    print!("{}", maze);
    let distance = maze.walk_main_loop().map(|len| len / 2)?;
    maze.see_outside()?;
    print!("{}", maze);
    let enclosed_area = maze.count_no_pipes();
    Ok((distance, enclosed_area))
}

fn main() {
    let input = include_str!("../input/day10_maze.txt");

    let start = std::time::Instant::now();
    println!("{:?}", solve_maze(input));
    let duration = start.elapsed();

    println!("Time elapsed is: {duration:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = r#"
            .....
            .S-7.
            .|.|.
            .L-J.
            .....
        "#;
        assert_eq!(solve_maze(input).unwrap(), (4, 1));
    }

    #[test]
    fn test_example_1_1() {
        let input = r#"
            .....
            .F-7.
            .S.|.
            .L-J.
            .....
        "#;
        assert_eq!(solve_maze(input).unwrap(), (4, 1));
    }

    #[test]
    fn test_example_2() {
        let input = r#"
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
        "#;
        assert_eq!(solve_maze(input).unwrap(), (8, 1));
    }

    #[test]
    fn test_example_3() {
        let input = r#"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        "#;
        assert_eq!(solve_maze(input).unwrap(), (23, 4));
    }

    #[test]
    fn test_example_4() {
        let input = r#"
            OF----7F7F7F7F-7OOOO
            O|F--7||||||||FJOOOO
            O||OFJ||||||||L7OOOO
            FJL7L7LJLJ||LJIL-7OO
            L--JOL7IIILJS7F-7L7O
            OOOOF-JIIF7FJ|L7L7L7
            OOOOL7IF7||L7|IL7L7|
            OOOOO|FJLJ|FJ|F7|OLJ
            OOOOFJL-7O||O||||OOO
            OOOOL---JOLJOLJLJOOO
        "#;
        assert_eq!(solve_maze(input).unwrap(), (23, 9));
    }
}
