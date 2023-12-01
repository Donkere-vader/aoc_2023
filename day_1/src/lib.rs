fn extract_numbers_from_line(line: &str) -> Vec<usize> {
    line.chars()
        .filter(char::is_ascii_digit)
        .map(|c| c as usize - 48)
        .collect()
}

fn extract_text_numbers_and_numbers_from_line(line: &str) -> Vec<usize> {
    let mut numbers = Vec::new();
    let mut i = 0;

    while i < line.len() {
        let slice = &line[i..(i + 5).min(line.len())];

        let mut text_number = None;

        for (text, digit) in vec![
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ] {
            if slice.starts_with(text) {
                text_number = Some(digit);
                break;
            }
        }

        if let Some(num) = text_number {
            numbers.push(num);
        } else {
            let first_char = slice.chars().take(1).collect::<Vec<char>>()[0];
            if first_char.is_ascii_digit() {
                numbers.push(first_char as usize - 48);
            }
        }

        i += 1;
    }

    numbers
}

fn parse_input(input: &str, extract_fn: &dyn Fn(&str) -> Vec<usize>) -> Vec<Vec<usize>> {
    let mut numbers = Vec::new();

    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        }

        let extracted_numbers = extract_fn(line);
        numbers.push(extracted_numbers);
    }

    numbers
}

pub fn part_1(input: String) -> usize {
    let numbers = parse_input(&input, &extract_numbers_from_line);

    numbers.iter().map(|v| v[0] * 10 + v[v.len() - 1]).sum()
}

pub fn part_2(input: String) -> usize {
    let numbers = parse_input(&input, &extract_text_numbers_and_numbers_from_line);

    numbers.iter().map(|v| v[0] * 10 + v[v.len() - 1]).sum()
}

#[cfg(test)]
mod tests {
    use common::get_input_file;

    use super::*;

    #[test]
    fn extra_extract_text_and_numbers() {
        assert_eq!(
            extract_text_numbers_and_numbers_from_line("123one"),
            vec![1, 2, 3, 1]
        );
        assert_eq!(
            extract_text_numbers_and_numbers_from_line("five2eight5"),
            vec![5, 2, 8, 5]
        );
        assert_eq!(
            extract_text_numbers_and_numbers_from_line("oneoneoneone"),
            vec![1, 1, 1, 1]
        );
        assert_eq!(
            extract_text_numbers_and_numbers_from_line("fouronethree3"),
            vec![4, 1, 3, 3]
        );
        assert_eq!(
            extract_text_numbers_and_numbers_from_line("1111"),
            vec![1, 1, 1, 1]
        );
        assert_eq!(
            extract_text_numbers_and_numbers_from_line("twonefour"),
            vec![2, 1, 4]
        );

        assert_eq!(extract_text_numbers_and_numbers_from_line("one"), vec![1]);
        assert_eq!(extract_text_numbers_and_numbers_from_line("two"), vec![2]);
        assert_eq!(extract_text_numbers_and_numbers_from_line("three"), vec![3]);
        assert_eq!(extract_text_numbers_and_numbers_from_line("four"), vec![4]);
        assert_eq!(extract_text_numbers_and_numbers_from_line("five"), vec![5]);
        assert_eq!(extract_text_numbers_and_numbers_from_line("six"), vec![6]);
        assert_eq!(extract_text_numbers_and_numbers_from_line("seven"), vec![7]);
        assert_eq!(extract_text_numbers_and_numbers_from_line("eight"), vec![8]);
        assert_eq!(extract_text_numbers_and_numbers_from_line("nine"), vec![9]);
        assert_eq!(extract_text_numbers_and_numbers_from_line(""), vec![]);
    }

    #[test]
    fn part_1_test() {
        let test_file_content = get_input_file(1, "test1.txt").unwrap();
        let result = part_1(test_file_content);
        assert_eq!(result, 142);
    }

    #[test]
    fn part_1_actual() {
        let test_file_content = get_input_file(1, "input.txt").unwrap();
        let result = part_1(test_file_content);
        assert_eq!(result, 55386);
    }

    #[test]
    fn part_2_test() {
        let test_file_content = get_input_file(1, "test2.txt").unwrap();
        let result = part_2(test_file_content);
        assert_eq!(result, 281);
    }

    #[test]
    fn part_2_actual() {
        let test_file_content = get_input_file(1, "input.txt").unwrap();
        let result = part_2(test_file_content);
        assert_eq!(result, 54824);
    }
}
