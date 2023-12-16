use std::collections::HashMap;


#[derive(Debug)]
struct Card {
    pub winning_numbers: Vec<usize>,
    pub cards_numbers: Vec<usize>,
}

fn parse_input(input: &str) -> Vec<Card> {
    let mut cards = Vec::new();

    for line in input.split('\n') {
        if line.is_empty() { continue }

        let splitted_line: Vec<&str> = line.split(':').collect();
        let numbers_string = splitted_line[1];

        let mut new_card = Card { winning_numbers: Vec::new(), cards_numbers: Vec::new() };

        let mut num = 0;
        let mut array = &mut new_card.winning_numbers;
        for c in numbers_string.chars() {
            let transformed_char = c as i32 - 48;
            if (0..=9).contains(&transformed_char) {
                num = num*10 + transformed_char;
            } else if num != 0 {
                array.push(num as usize);
                num = 0;
            } else if c == '|' {
                array = &mut new_card.cards_numbers;
            }
        }

        if num != 0 {
            array.push(num as usize);
        }

        cards.push(new_card);
    }

    cards
}

pub fn part_1(input: String) -> usize {
    let cards = parse_input(&input);

    cards.iter()
        .map(|c| c.cards_numbers.iter()
            .filter(|n| c.winning_numbers.contains(n))
            .count())
        .filter(|n| *n > 0)
        .map(|n| 2usize.pow((n - 1) as u32))
        .sum()
}

pub fn part_2(input: String) -> usize {
    let cards = parse_input(&input);

    let mut scores: HashMap<usize, usize> = HashMap::new();

    for (index, card) in cards.iter().enumerate().rev() {
        let amount_of_winning_cards = card.cards_numbers.iter()
                .filter(|n| card.winning_numbers.contains(n))
                .count();

        let mut score = 1;
        for i in 1..(amount_of_winning_cards + 1) {
            score += scores.get(&(i + index)).unwrap();
        }
        
        scores.insert(index, score);
    }

    scores.values()
        .sum()
}

#[cfg(test)]
mod test {
    use common::get_input_file;

    use super::{part_1, part_2};

    #[test]
    fn part_1_test() {
        let input = get_input_file(4, "test.txt").unwrap();
        let result = part_1(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn part_1_actual() {
        let input = get_input_file(4, "input.txt").unwrap();
        let result = part_1(input);
        assert_eq!(result, 27059);
    }

    #[test]
    fn part_2_test() {
        let input = get_input_file(4, "test.txt").unwrap();
        let result = part_2(input);
        assert_eq!(result, 30);
    }

    #[test]
    fn part_2_actual() {
        let input = get_input_file(4, "input.txt").unwrap();
        let result = part_2(input);
        assert_eq!(result, 5744979);
    }
}