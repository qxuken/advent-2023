#[derive(Debug)]
struct SeedRange {
    start: usize,
    len: usize,
}

impl SeedRange {
    fn new(start: usize, len: usize) -> SeedRange {
        SeedRange { start, len }
    }
}

impl SeedRange {
    fn extract_ranges<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Vec<SeedRange> {
        match lines.find(|s| s.starts_with("seeds:")) {
            Some(s) => {
                let mut ranges: Vec<SeedRange> = vec![];
                let mut iter = s
                    .split_ascii_whitespace()
                    .filter_map(|n| n.parse::<usize>().ok());
                while let (Some(start), Some(len)) = (iter.next(), iter.next()) {
                    ranges.push(SeedRange::new(start, len));
                }
                ranges.sort_by_key(|r| r.start);
                ranges
            }
            None => vec![],
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

    fn can_convert(&self, number: usize) -> bool {
        number >= self.source_start && number < self.source_start + self.len
    }

    fn convert(&self, number: usize) -> Option<usize> {
        if self.can_convert(number) {
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

    fn find<F>(&self, f: F) -> Option<&ConversionMapRange>
    where
        F: Fn(&&ConversionMapRange) -> bool,
    {
        self.ranges.iter().find(f)
    }
}

impl FromIterator<[usize; 3]> for ConversionMap {
    fn from_iter<T: IntoIterator<Item = [usize; 3]>>(iter: T) -> Self {
        let ranges: Vec<ConversionMapRange> = iter.into_iter().map(|r| r.into()).collect();
        Self::new(ranges)
    }
}

impl ConversionMap {
    fn fill_gaps(&mut self, highest_number: usize) {
        if self.ranges.is_empty() {
            return;
        }
        self.ranges.sort_by_key(|r| r.source_start);
        let mut number = 0;
        let mut ranges: Vec<ConversionMapRange> = vec![];
        for range in self.ranges.iter().copied() {
            if range.source_start != number && !ranges.iter().any(|r| r.can_convert(number)) {
                ranges.push(ConversionMapRange::new(
                    number,
                    number,
                    range.source_start - number,
                ));
            }
            number = range.source_start + range.len;
            ranges.push(range);
        }
        if highest_number > number {
            ranges.push(ConversionMapRange::new(
                number,
                number,
                highest_number - number,
            ));
        }
        self.ranges = ranges;
    }

    fn extract<'a>(
        lines: &mut impl Iterator<Item = &'a str>,
        highest_number: usize,
    ) -> Option<ConversionMap> {
        let mut map: ConversionMap = lines
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
        map.fill_gaps(highest_number);
        Some(map).filter(|m| !m.is_empty())
    }
}

fn min_location_with_ranges(input: &str) -> usize {
    let mut lines = input.split('\n').map(|s| s.trim());
    let seeds = SeedRange::extract_ranges(&mut lines);
    let last_seed = seeds.last().unwrap();
    let highest_number = last_seed.start + last_seed.len;
    let mut maps = vec![];
    while let Some(map) = ConversionMap::extract(&mut lines, highest_number) {
        maps.push(map);
    }
    seeds
        .into_iter()
        .map(|s| {
            let mut lowest = usize::MAX;
            let mut remaining = s.len;
            while remaining > 0 {
                let mut min_range = remaining;
                let mut current = s.start + s.len - remaining;
                for map in maps.iter() {
                    if let Some(range) = map.find(|r| r.can_convert(current)) {
                        min_range = min_range.min(range.len - (current - range.source_start));
                        current = range.convert(current).unwrap();
                    } else {
                        min_range = 1;
                    }
                }
                lowest = lowest.min(current);
                remaining -= min_range;
            }
            lowest
        })
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("../input/day5_fertyseed.txt");

    println!("{}", min_location_with_ranges(input));
}

#[cfg(test)]
mod tests {
    use super::*;

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
