use std::collections::HashMap;

#[derive(Debug)]
pub struct NumberTrie {
    pub number: Option<usize>,
    pub children: HashMap<char, NumberTrie>,
}

impl NumberTrie {
    pub fn new() -> Self {
        NumberTrie {
            number: None,
            children: HashMap::new(),
        }
    }
}

impl Default for NumberTrie {
    fn default() -> Self {
        NumberTrie::new()
    }
}

impl From<Vec<(&str, usize)>> for NumberTrie {
    fn from(value: Vec<(&str, usize)>) -> Self {
        let mut root = NumberTrie::default();

        for (name, value) in value {
            root.append(name, value);
        }
        root
    }
}

impl NumberTrie {
    pub fn append(&mut self, value: &str, number: usize) {
        if value.is_empty() {
            return;
        }
        let mut node = self;
        for char in value.chars() {
            node = node.children.entry(char).or_default()
        }
        node.number = Some(number);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_trie() -> NumberTrie {
        vec![
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]
        .into()
    }

    #[test]
    fn basic_case() {
        let tr = create_trie();

        assert!(!tr.children.contains_key(&'b'));
        assert_eq!(
            tr.children
                .get(&'f')
                .and_then(|node| node.children.get(&'o'))
                .and_then(|node| node.children.get(&'u'))
                .and_then(|node| node.children.get(&'r'))
                .and_then(|node| node.number),
            Some(4)
        );
    }
}
