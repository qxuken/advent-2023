use rayon::prelude::*;

#[derive(Debug)]
struct ConversionMapRange {
    destination_start: usize,
    source_start: usize,
    len: usize,
}

impl ConversionMapRange {
    fn new(destination_start: usize, source_start: usize, len: usize) -> Self {
        Self {
            destination_start,
            source_start,
            len,
        }
    }

    fn has_conversions(&self, number: usize) -> bool {
        number >= self.source_start && number < self.source_start + self.len
    }

    fn convert(&self, number: usize) -> Option<usize> {
        if self.has_conversions(number) {
            Some(self.destination_start + number - self.source_start)
        } else {
            None
        }
    }
}

impl From<[usize; 3]> for ConversionMapRange {
    fn from(value: [usize; 3]) -> Self {
        Self::new(value[0], value[1], value[2])
    }
}

#[derive(Debug)]
struct ConversionMap {
    ranges: Vec<ConversionMapRange>,
}

impl ConversionMap {
    fn new(ranges: Vec<ConversionMapRange>) -> Self {
        Self { ranges }
    }

    fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    fn convert(&self, number: usize) -> usize {
        self.ranges
            .iter()
            .find(|r| r.has_conversions(number))
            .and_then(|m| m.convert(number))
            .unwrap_or(number)
    }
}

impl FromIterator<[usize; 3]> for ConversionMap {
    fn from_iter<T: IntoIterator<Item = [usize; 3]>>(iter: T) -> Self {
        let ranges: Vec<ConversionMapRange> = iter.into_iter().map(|r| r.into()).collect();
        Self::new(ranges)
    }
}

fn extract_seeds<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Vec<usize> {
    match lines.find(|s| s.starts_with("seeds:")) {
        Some(s) => s
            .split_ascii_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect(),
        None => vec![],
    }
}

fn extract_seed_ranges<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Vec<usize> {
    match lines.find(|s| s.starts_with("seeds:")) {
        Some(s) => {
            let mut ranges: Vec<usize> = vec![];
            let mut iter = s
                .split_ascii_whitespace()
                .filter_map(|n| n.parse::<usize>().ok());
            while let (Some(start), Some(len)) = (iter.next(), iter.next()) {
                ranges.extend(start..start + len);
            }
            ranges
        }
        None => vec![],
    }
}

fn extract_map<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Option<ConversionMap> {
    let map: ConversionMap = lines
        .by_ref()
        .skip_while(|s| !s.starts_with(|ch: char| ch.is_ascii_digit()))
        .take_while(|s| !s.is_empty())
        .map(|s| {
            s.split_ascii_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect::<Vec<usize>>()
        })
        .filter(|vec| vec.len() == 3)
        .map(|vec| [vec[0], vec[1], vec[2]])
        .collect();
    Some(map).filter(|m| !m.is_empty())
}

fn min_location(input: &str) -> usize {
    let mut lines = input.split('\n').map(|s| s.trim());
    let mut seeds = extract_seeds(&mut lines);
    while let Some(map) = extract_map(&mut lines) {
        for seed in seeds.iter_mut() {
            *seed = map.convert(*seed);
        }
    }
    seeds.into_iter().min().unwrap()
}

fn min_location_with_ranges(input: &str) -> usize {
    let mut lines = input.split('\n').map(|s| s.trim());
    let seeds = extract_seed_ranges(&mut lines);
    let mut maps = vec![];
    while let Some(map) = extract_map(&mut lines) {
        maps.push(map);
    }
    seeds
        .into_par_iter()
        .map(|x| maps.iter().fold(x, |acc, map| map.convert(acc)))
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("../input/day5_fertyseed.txt");

    println!("{}", min_location(input));
    println!("{}", min_location_with_ranges(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_location() {
        let input = r#"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4
        "#;
        assert_eq!(min_location(input), 35);
    }

    #[test]
    fn test_min_location_with_ranges() {
        let input = r#"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4
        "#;
        assert_eq!(min_location_with_ranges(input), 46);
    }
}
