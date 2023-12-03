#[derive(PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn is_exceeded(&self, count: usize, max_r: usize, max_g: usize, max_b: usize) -> bool {
        match self {
            Self::Red => count > max_r,
            Self::Green => count > max_g,
            Self::Blue => count > max_b,
        }
    }
}

fn find_game_id(chars: &mut impl Iterator<Item = char>) -> Option<usize> {
    chars
        .by_ref()
        .skip_while(|ch| !ch.is_ascii_digit())
        .take_while(|&ch| ch != ':')
        .collect::<String>()
        .parse()
        .ok()
}

fn get_color_count(chars: &mut impl Iterator<Item = char>) -> Option<(Color, usize)> {
    let count = chars
        .by_ref()
        .skip_while(|ch| !ch.is_ascii_digit())
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>()
        .parse()
        .ok();
    let color = chars
        .by_ref()
        .skip_while(|ch| !ch.is_ascii_alphabetic())
        .take_while(|ch| ch.is_ascii_alphabetic())
        .collect::<String>();
    let color = match color.as_ref() {
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "blue" => Some(Color::Blue),
        _ => return None,
    };
    color.zip(count)
}

fn sum_of_possible_games(inp: &str, max_r: usize, max_g: usize, max_b: usize) -> usize {
    let mut sum = 0;
    'line_loop: for line in inp.split('\n') {
        let mut chars = line.chars();
        if let Some(game_id) = find_game_id(&mut chars) {
            while let Some((color, count)) = get_color_count(&mut chars) {
                if color.is_exceeded(count, max_r, max_g, max_b) {
                    continue 'line_loop;
                }
            }
            sum += game_id;
        }
    }
    sum
}

fn sum_of_min_required_cubes_power(inp: &str) -> usize {
    let mut sum = 0;
    for line in inp.split('\n') {
        let mut chars = line.chars();
        if find_game_id(&mut chars).is_some() {
            let mut max_r = 0;
            let mut max_g = 0;
            let mut max_b = 0;
            while let Some((color, count)) = get_color_count(&mut chars) {
                match color {
                    Color::Red => max_r = max_r.max(count),
                    Color::Green => max_g = max_g.max(count),
                    Color::Blue => max_b = max_b.max(count),
                }
            }
            sum += max_r * max_g * max_b;
        }
    }
    sum
}

fn main() {
    let input = include_str!("../input/day2_cube_conundrum.txt");

    println!("{}", sum_of_possible_games(input, 12, 13, 14));
    println!("{}", sum_of_min_required_cubes_power(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_possible_games() {
        let input = r#"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;
        assert_eq!(sum_of_possible_games(input, 12, 13, 14), 1 + 2 + 5);
    }

    #[test]
    fn test_sum_of_min_cubes_power() {
        let input = r#"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;
        assert_eq!(
            sum_of_min_required_cubes_power(input),
            48 + 12 + 1560 + 630 + 36
        );
    }
}
