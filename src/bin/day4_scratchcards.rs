use std::collections::{HashMap, HashSet};

fn find_card_id(chars: &mut impl Iterator<Item = char>) -> Option<usize> {
    chars
        .by_ref()
        .skip_while(|ch| !ch.is_ascii_digit())
        .take_while(|&ch| ch != ':')
        .collect::<String>()
        .parse()
        .ok()
}

fn parse_number_buffer(num: &[char]) -> Option<usize> {
    num.iter().collect::<String>().parse().ok()
}

fn extract_winning_numbers(chars: &mut impl Iterator<Item = char>) -> HashSet<usize> {
    chars
        .by_ref()
        .skip_while(|ch| !ch.is_ascii_digit())
        .take_while(|&ch| ch != '|')
        .fold((HashSet::new(), vec![]), |(mut set, mut buffer), ch| {
            if ch.is_ascii_digit() {
                buffer.push(ch);
            } else {
                if let Some(num) = parse_number_buffer(&buffer) {
                    set.insert(num);
                }
                buffer.clear();
            }
            (set, buffer)
        })
        .0
}

fn get_wins_count(
    winning_numbers: &HashSet<usize>,
    chars: &mut impl Iterator<Item = char>,
) -> usize {
    let mut count = 0;
    let mut buffer = vec![];
    for ch in chars {
        if ch.is_ascii_digit() {
            buffer.push(ch);
        } else {
            if parse_number_buffer(&buffer).is_some_and(|num| winning_numbers.contains(&num)) {
                count += 1;
            }
            buffer.clear();
        }
    }
    if parse_number_buffer(&buffer).is_some_and(|num| winning_numbers.contains(&num)) {
        count += 1;
    }
    count
}

fn sum_of_wins(input: &str) -> usize {
    let mut sum = 0;
    for line in input.split('\n') {
        let mut chars = line.chars();
        if find_card_id(&mut chars).is_some() {
            let winning_numbers = extract_winning_numbers(&mut chars);
            let wins_count = get_wins_count(&winning_numbers, &mut chars);

            if wins_count > 0 {
                let pow = (wins_count as u32) - 1;
                let score = 2usize.pow(pow);
                sum += score;
            }
        }
    }
    sum
}

fn sum_of_cards(input: &str) -> usize {
    let mut cards_counter = HashMap::new();
    for line in input.split('\n') {
        let mut chars = line.chars();
        if let Some(card_id) = find_card_id(&mut chars) {
            let count = *cards_counter
                .entry(card_id)
                .and_modify(|c| *c += 1)
                .or_insert(1);

            let winning_numbers = extract_winning_numbers(&mut chars);
            let wins_count = get_wins_count(&winning_numbers, &mut chars);

            for i in card_id + 1..=card_id + wins_count {
                *cards_counter.entry(i).or_insert(0) += count;
            }
        }
    }
    cards_counter.values().sum()
}

fn main() {
    let input = include_str!("../input/day4_scratchcards.txt");

    println!("{}", sum_of_wins(input));
    println!("{}", sum_of_cards(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_wins() {
        let input = r#"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#;
        assert_eq!(sum_of_wins(input), 13);
    }

    #[test]
    fn test_sum_of_cards() {
        let input = r#"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#;
        assert_eq!(sum_of_cards(input), 30);
    }
}
