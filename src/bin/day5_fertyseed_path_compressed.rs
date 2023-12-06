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

#[derive(Debug, Clone)]
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

    // s   t
    // 1---|  1
    // 2---|  2--
    // 3--x|> 3--
    // 4  ||> 4--
    // 5  |>  5
    // ==== step 1
    // source range dest = 3, source = 1, len = 3
    // remain = 3, curr = (s.dest + (s.len - remain)) -> 3
    // target range start = 2, len = 3
    // min = min(remain -> 3, (t.len - (curr - t.start)) -> 2)
    // new range start = s.start + (s.len - remain), len = min, dest = t.dest + (curr - t.start)
    // ==== step 2
    // source range dest = 3, source = 1, len = 3
    // remain = 1, curr = 5
    // target range range start = 5, len = ...
    fn compress(&self, other: &ConversionMap) -> ConversionMap {
        let mut ranges = vec![];
        for source_range in self.ranges.iter().copied() {
            let mut remaining = source_range.len;
            while remaining > 0 {
                let source_diff = source_range.len - remaining;
                let current = source_range.destination_start + source_diff;

                if let Some(target_range) = other.find(|r| r.can_convert(current)) {
                    let target_diff = current - target_range.source_start;
                    let min_range = remaining.min(target_range.len - target_diff);
                    remaining -= min_range;
                    ranges.push(ConversionMapRange::new(
                        target_range.destination_start + target_diff,
                        source_range.source_start + source_diff,
                        min_range,
                    ));
                } else {
                    remaining -= 1;
                    ranges.push(ConversionMapRange::new(
                        source_range.destination_start + source_diff,
                        source_range.source_start + source_diff,
                        1,
                    ));
                }
            }
        }
        for i in (1..ranges.len()).rev() {
            let should_merge = {
                let prev_range = &ranges[i - 1];
                let curr_range = &ranges[i];
                curr_range.destination_start as isize - prev_range.len as isize
                    == prev_range.destination_start as isize
                    && curr_range.source_start as isize - prev_range.len as isize
                        == prev_range.source_start as isize
            };
            if should_merge {
                ranges[i - 1].len += ranges[i].len;
                ranges.remove(i);
            }
        }
        ConversionMap::new(ranges)
    }
}

fn min_location_with_ranges(input: &str) -> usize {
    let mut lines = input.split('\n').map(|s| s.trim());
    let seeds = SeedRange::extract_ranges(&mut lines);
    let last_seed = seeds.last().expect("no seeds found");
    let highest_number = last_seed.start + last_seed.len;
    let mut compressed_map: Option<ConversionMap> = None;

    let start = std::time::Instant::now();
    while let Some(map) = ConversionMap::extract(&mut lines, highest_number) {
        compressed_map = compressed_map.map(|m| m.compress(&map)).or(Some(map));
    }
    let compressed_map = compressed_map.expect("Map is empty");
    let duration = start.elapsed();
    println!("Map compressed in {:?}", duration);

    for path in compressed_map.ranges.iter() {
        println!(
            "{:010} -> {:010} | {:010}",
            path.source_start, path.destination_start, path.len
        );
    }

    let start = std::time::Instant::now();
    let min = seeds
        .into_iter()
        .map(|s| {
            let mut lowest = usize::MAX;
            let mut remaining = s.len;
            while remaining > 0 {
                let mut min_range = remaining;
                let mut current = s.start + s.len - remaining;
                if let Some(range) = compressed_map.find(|r| r.can_convert(current)) {
                    min_range = min_range.min(range.len - (current - range.source_start));
                    current = range.convert(current).unwrap();
                } else {
                    min_range = 1;
                }
                lowest = lowest.min(current);
                remaining -= min_range;
            }
            lowest
        })
        .min()
        .unwrap();
    let duration = start.elapsed();
    println!("Solution ready in {:?}", duration);
    min
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
