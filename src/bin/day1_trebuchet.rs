use advent_2023::number_trie::NumberTrie;
use once_cell::sync::Lazy;

const INPUT: &str = include_str!("../input/day1_trebuchet.txt");

const SPELLED_NUMBERS: [(&str, usize); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

static SPELLED_NUMBERS_TRIE: Lazy<NumberTrie> = Lazy::new(|| SPELLED_NUMBERS.to_vec().into());
static REVERSED_SPELLED_NUMBERS_TRIE: Lazy<NumberTrie> = Lazy::new(|| {
    SPELLED_NUMBERS
        .iter()
        .fold(NumberTrie::default(), |mut root, &(spelled, value)| {
            let reversed = spelled.chars().rev().collect::<String>();
            root.append(&reversed, value);
            root
        })
});

fn find_number(chars: &mut impl Iterator<Item = char>, trie: &NumberTrie) -> Option<String> {
    let mut trie_nodes: Vec<&NumberTrie> = vec![];
    for ch in chars {
        if ch.is_ascii_digit() {
            return Some(ch.to_string());
        }
        for i in (0..trie_nodes.len()).rev() {
            if let Some(child) = trie_nodes[i].children.get(&ch) {
                if let Some(number) = child.number {
                    return Some(number.to_string());
                }

                trie_nodes[i] = child;
            } else {
                trie_nodes.remove(i);
            }
        }
        if let Some(node) = trie.children.get(&ch) {
            trie_nodes.push(node)
        }
    }
    None
}

pub fn solution(inp: &str, trie: &NumberTrie, rev_trie: &NumberTrie) -> usize {
    let mut sum = 0;
    for line in inp.split('\n') {
        let mut chars = line.chars();
        let first = find_number(&mut chars, trie);
        let last = find_number(&mut chars.rev(), rev_trie);
        if let (Some(left), Some(right)) = (first.as_ref(), last.as_ref().or(first.as_ref())) {
            let num: usize = format!("{}{}", left, right).parse().unwrap();
            sum += num;
        }
    }
    sum
}

fn main() {
    if std::env::args()
        .nth(1)
        .is_some_and(|val| val == "no_spelled")
    {
        let empty_trie = NumberTrie::default();
        println!("{}", solution(INPUT, &empty_trie, &empty_trie))
    } else {
        println!(
            "{}",
            solution(INPUT, &SPELLED_NUMBERS_TRIE, &REVERSED_SPELLED_NUMBERS_TRIE)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_solution() {
        let input = r#"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "#;
        assert_eq!(
            solution(input, &SPELLED_NUMBERS_TRIE, &REVERSED_SPELLED_NUMBERS_TRIE),
            12 + 38 + 15 + 77
        );
    }

    #[test]
    fn test_spelled_digits_solution() {
        let input = r#"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "#;
        assert_eq!(
            solution(input, &SPELLED_NUMBERS_TRIE, &REVERSED_SPELLED_NUMBERS_TRIE),
            29 + 83 + 13 + 24 + 42 + 14 + 76
        );
    }
}
