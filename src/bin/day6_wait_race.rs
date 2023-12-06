#[derive(Debug)]
struct Race {
    time: usize,
    record_distance: usize,
}

impl Race {
    fn new(time: usize, record: usize) -> Self {
        Self {
            time,
            record_distance: record,
        }
    }
}

impl Race {
    fn find_min_time(&self, start: usize, end: usize) -> Option<usize> {
        let middle = (end + start) / 2;
        let distance = middle * (self.time - middle);
        println!(
            "{:?} -> mid {} dist {} {}",
            start..=end,
            middle,
            distance,
            match self.record_distance.cmp(&distance) {
                std::cmp::Ordering::Equal => ">",
                std::cmp::Ordering::Greater => ">",
                std::cmp::Ordering::Less => "<",
            }
        );
        if start == end {
            return Some(start).filter(|_| distance > self.record_distance);
        }
        if self.record_distance >= distance {
            self.find_min_time(middle + 1, end)
                .or(Some(middle).filter(|_| distance > self.record_distance))
        } else {
            self.find_min_time(start, middle - 1)
                .or(Some(middle).filter(|_| distance > self.record_distance))
        }
    }

    fn find_max_time(&self, start: usize, end: usize) -> Option<usize> {
        let middle = (end + start) / 2;
        let distance = middle * (self.time - middle);
        println!(
            "{:?} -> mid {} dist {} {}",
            start..=end,
            middle,
            distance,
            match distance.cmp(&self.record_distance) {
                std::cmp::Ordering::Equal => ">",
                std::cmp::Ordering::Greater => ">",
                std::cmp::Ordering::Less => "<",
            }
        );
        if start == end {
            return Some(start).filter(|_| distance > self.record_distance);
        }
        if distance >= self.record_distance {
            self.find_max_time(middle + 1, end)
                .or(Some(middle).filter(|_| distance > self.record_distance))
        } else {
            self.find_max_time(start, middle - 1)
                .or(Some(middle).filter(|_| distance > self.record_distance))
        }
    }

    fn get_wins_count(&self) -> usize {
        println!("== {} rec {:?}", self.time, self.record_distance);
        let min = self.find_min_time(1, self.time).expect("no min");
        println!("min {:?}", min);
        let max = self.find_max_time(1, self.time).expect("no max");
        println!("max {:?}", max);

        max - min + 1
    }
}

pub fn power_of_race(input: &str) -> usize {
    let lines = input
        .trim()
        .split('\n')
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    if lines.len() != 2 || lines[0].len() != lines[1].len() {
        panic!("Input error");
    }
    let mut power = 1;
    for i in 0..lines[0].len() {
        let race = Race::new(lines[0][i], lines[1][i]);
        let wins_count = race.get_wins_count();
        println!("wins {:?}", wins_count);
        power *= wins_count;
    }
    println!("power {:?}", power);
    power
}

pub fn smashed_race(input: &str) -> usize {
    let lines = input
        .trim()
        .split('\n')
        .filter_map(|line| {
            line.chars()
                .filter(char::is_ascii_digit)
                .collect::<String>()
                .parse()
                .ok()
        })
        .collect::<Vec<usize>>();
    if lines.len() != 2 {
        panic!("Input error");
    }
    let race = Race::new(lines[0], lines[1]);
    let wins_count = race.get_wins_count();
    println!("wins {:?}", wins_count);

    wins_count
}

fn main() {
    let input = include_str!("../input/day6_wait_race.txt");

    let start = std::time::Instant::now();
    power_of_race(input);
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);

    println!("======");
    let start = std::time::Instant::now();
    smashed_race(input);
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_of_race() {
        let input = r#"
            Time:      7  15   30
            Distance:  9  40  200
        "#;
        assert_eq!(power_of_race(input), 4 * 8 * 9);
    }

    #[test]
    fn test_smashed_race() {
        let input = r#"
            Time:      7  15   30
            Distance:  9  40  200
        "#;
        assert_eq!(smashed_race(input), 71503);
    }
}
