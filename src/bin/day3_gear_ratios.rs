use std::collections::VecDeque;

fn get_digit_char(line: &str, i: usize) -> Option<char> {
    line.get(i..i + 1)
        .and_then(|s| s.chars().next())
        .filter(char::is_ascii_digit)
}

fn extract_number(line: &mut String, i: usize) -> Option<usize> {
    let mut min = i;
    let mut max = i + 1;
    let mut vec_deque = VecDeque::new();

    for j in (0..i).rev() {
        if let Some(digit) = get_digit_char(line, j) {
            vec_deque.push_front(digit);
        } else {
            min = j + 1;
            break;
        }
        if j == 0 {
            min = 0;
        }
    }

    for j in i..line.len() {
        if let Some(digit) = get_digit_char(line, j) {
            vec_deque.push_back(digit);
        } else {
            max = j;
            break;
        }
        if j == line.len() - 1 {
            max = line.len();
        }
    }

    let range = min..max;
    let len = &range.len();

    line.replace_range(range, &".".repeat(*len));

    vec_deque.into_iter().collect::<String>().parse().ok()
}

fn scan_surrounds(lines: &mut [String], i: usize, j: usize) -> Vec<usize> {
    let mut res = vec![];

    // (i - 1, j - 1) (i - 1, j) (i - 1, j + 1)
    // (i,     j + 1) (i,     j) (i,     j + 1)
    // (i + 1, j - 1) (i + 1, j) (i + 1, j + 1)
    for i in (i - 1)..=i + 1 {
        for j in (j - 1)..=j + 1 {
            if lines.get(i).and_then(|l| get_digit_char(l, j)).is_some() {
                if let Some(number) = extract_number(&mut lines[i], j) {
                    res.push(number);
                }
            }
        }
    }
    res
}

fn sum_of_parts(input: &str) -> usize {
    let mut sum = 0;
    let mut lines = input
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<Vec<String>>();
    for i in 0..lines.len() {
        let line = lines[i].clone();
        for (j, ch) in line.chars().enumerate() {
            if ch == '.' || ch.is_ascii_digit() {
                continue;
            }
            let parts = scan_surrounds(&mut lines, i, j);
            sum += parts.iter().sum::<usize>();
        }
    }
    sum
}

fn sum_of_gears_ratio(input: &str) -> usize {
    let mut sum = 0;
    let mut lines = input
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<Vec<String>>();
    for i in 0..lines.len() {
        let line = lines[i].clone();
        for (j, ch) in line.chars().enumerate() {
            if ch != '*' {
                continue;
            }
            let parts = scan_surrounds(&mut lines, i, j);

            if parts.len() != 2 {
                continue;
            }
            sum += parts.iter().product::<usize>();
        }
    }
    sum
}

fn main() {
    let input = include_str!("../input/day3_gear_ratios.txt");

    println!("{}", sum_of_parts(input));
    println!("{}", sum_of_gears_ratio(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_parts() {
        let input = r#"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "#;
        assert_eq!(sum_of_parts(input), 4361);
    }

    #[test]
    fn test_sum_of_gears_ratio() {
        let input = r#"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "#;
        assert_eq!(sum_of_gears_ratio(input), 467835);
    }

    #[test]
    fn test_extract_number_start() {
        let mut scan_str = "123.123...123".to_string();
        assert_eq!(extract_number(&mut scan_str, 0), Some(123));
        assert_eq!(scan_str, "....123...123");
    }

    #[test]
    fn test_extract_number_middle() {
        let mut scan_str = "123.123...123".to_string();
        assert_eq!(extract_number(&mut scan_str, 5), Some(123));
        assert_eq!(scan_str, "123.......123");
    }

    #[test]
    fn test_extract_number_end() {
        let mut scan_str = "123.123...123".to_string();
        assert_eq!(extract_number(&mut scan_str, 11), Some(123));
        assert_eq!(scan_str, "123.123......");
    }
}
