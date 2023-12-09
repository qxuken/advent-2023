use rayon::prelude::*;

pub fn extrapolation_vec(numbers: Vec<isize>) -> Vec<Vec<isize>> {
    let mut extrapolation = vec![numbers];
    while let Some(last) = extrapolation.last().filter(|v| !v.iter().all(|&n| n == 0)) {
        let mut row = Vec::with_capacity(last.len() - 1);
        for i in 1..last.len() {
            row.push(last[i] - last[i - 1])
        }
        extrapolation.push(row);
    }
    // println!();
    // for (i, row) in extrapolation.iter().enumerate() {
    //     println!(
    //         "{:size$}{}",
    //         "",
    //         row.iter().fold(String::new(), |mut s, n| {
    //             s.push_str(&format!("{:2}", n));
    //             s.push(' ');
    //             s
    //         }),
    //         size = i
    //     );
    // }
    extrapolation
}

pub fn extrapolate_next(extrapolation: &[Vec<isize>]) -> isize {
    extrapolation.iter().rev().filter_map(|r| r.last()).sum()
}
pub fn extrapolate_prev(extrapolation: &[Vec<isize>]) -> isize {
    extrapolation
        .iter()
        .rev()
        .filter_map(|r| r.first())
        .fold(0, |prev, curr| curr - prev)
}

pub fn extrapolation_sum(input: &str) -> (isize, isize) {
    input
        .trim()
        .split('\n')
        .par_bridge()
        .map(|line| {
            line.trim()
                .split(' ')
                .filter_map(|n| n.parse().ok())
                .collect()
        })
        .map(extrapolation_vec)
        .map(|e| (extrapolate_prev(&e), extrapolate_next(&e)))
        .reduce(
            || (0, 0),
            |(prev_sum, next_sum), (prev, next)| (prev_sum + prev, next_sum + next),
        )
}

fn main() {
    let input = include_str!("../input/day9_mirage.txt");

    let start = std::time::Instant::now();
    println!("{:?}", extrapolation_sum(input));
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = r#"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "#;
        assert_eq!(extrapolation_sum(input), (2, 114));
    }
}
