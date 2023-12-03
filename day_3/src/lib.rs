use std::{fmt::Display, collections::HashMap};


#[derive(PartialEq, Eq, Debug)]
enum MatrixValue {
    Number(usize),
    Cog,
    Symbol,
    Nothing,
}

impl MatrixValue {
    pub fn is_symbol_or_cog(&self) -> bool {
        match self {
            Self::Cog => true,
            Self::Symbol => true,
            _ => false,
        }
    }
}

impl Display for MatrixValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => f.write_str(&format!("{n}")),
            Self::Cog => f.write_str("*"),
            Self::Symbol => f.write_str("S"),
            Self::Nothing => f.write_str(" "),
        }
    }
}

impl From<char> for MatrixValue {
    fn from(value: char) -> Self {
        if value == '.' {
            return MatrixValue::Nothing;
        } else if value == '*' {
            return MatrixValue::Cog;
        } else if value.is_ascii_digit() {
            return MatrixValue::Number(value as usize - 48);
        } else {
            return MatrixValue::Symbol;
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<MatrixValue>> {
    input.split("\n")
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(MatrixValue::from).collect())
        .collect()
}

pub fn part_1(input: String) -> usize {
    let matrix = parse_input(&input);

    let mut part_numbers = Vec::new();

    let mut number = 0;
    let mut finding_number = true;
    let mut adjacent_symbol = false;

    for (y, row) in matrix.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            let now_adjacent_symbol = ((y as i32 - 1) >= 0 && matrix[(y as i32 - 1) as usize][x].is_symbol_or_cog())
                || (y + 1 < matrix.len() && matrix[y + 1][x].is_symbol_or_cog());

            match c {
                MatrixValue::Number(n) => {
                    finding_number = true;
                    number = number*10 + n;
                },
                MatrixValue::Cog |
                MatrixValue::Symbol => {
                    if finding_number {
                        part_numbers.push(number);
                        number = 0;
                        finding_number = false;
                    }
                    adjacent_symbol = true;
                },
                MatrixValue::Nothing => {
                    if finding_number && (adjacent_symbol || now_adjacent_symbol) {
                        part_numbers.push(number);
                    }
                    number = 0;
                    finding_number = false;
                    adjacent_symbol = false;
                },
            }

            if now_adjacent_symbol {
                adjacent_symbol = true;
            }
        }
    }

    part_numbers.iter().sum()
}

pub fn part_2(input: String) -> usize {
    let matrix = parse_input(&input);

    let mut cogs: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (y, row) in matrix.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == MatrixValue::Cog {
                cogs.insert((x, y), Vec::new());
            }
        }
    }

    let mut number = 0;
    let mut finding_number = true;
    let mut adjacent_cogs: Vec<(usize, usize)> = Vec::new();

    for (y, row) in matrix.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            let mut now_adjacent_cog = None;
            if (y as i32 - 1) >= 0 && matrix[(y as i32 - 1) as usize][x] == MatrixValue::Cog {
                now_adjacent_cog = Some((x, (y as i32 - 1) as usize));
            }
            if y + 1 < matrix.len() && matrix[y + 1][x] == MatrixValue::Cog {
                now_adjacent_cog = Some((x, y + 1));
            }

            match c {
                MatrixValue::Number(n) => {
                    finding_number = true;
                    number = number*10 + n;
                },
                MatrixValue::Cog => {
                    if finding_number {
                        if let Some(c) = now_adjacent_cog {
                            cogs.get_mut(&c).unwrap().push(number);
                        }
                        cogs.get_mut(&(x, y)).unwrap().push(number);
                        adjacent_cogs.iter()
                            .for_each(|c| cogs.get_mut(c).unwrap().push(number));
                        number = 0;
                        finding_number = false;
                    }
                    adjacent_cogs.clear();
                    adjacent_cogs.push((x, y));
                },
                MatrixValue::Symbol |
                MatrixValue::Nothing => {
                    if finding_number && (!adjacent_cogs.is_empty() || now_adjacent_cog.is_some()) {
                        // println!("{number} {adjacent_cogs:?}");
                        if let Some(c) = now_adjacent_cog {
                            cogs.get_mut(&c).unwrap().push(number);
                        }
                        adjacent_cogs.iter()
                            .for_each(|c| cogs.get_mut(c).unwrap().push(number));
                    }
                    number = 0;
                    finding_number = false;
                    adjacent_cogs.clear();
                },
            }

            if let Some(cog) = now_adjacent_cog {
                adjacent_cogs.push(cog);
            }
        }
    }

    cogs.values()
        .filter(|v| v.len() < 2)
        .map(|v| v.iter().sum::<usize>())
        .product()
}

#[cfg(test)]
mod tests {
    use common::get_input_file;

    use super::{part_1, part_2};

    #[test]
    fn part_1_test() {
        let input = get_input_file(3, "test.txt").unwrap();
        let answer = part_1(input);
        assert_eq!(answer, 4361);
    }

    #[test]
    fn part_1_actual() {
        let input = get_input_file(3, "input.txt").unwrap();
        let answer = part_1(input);
        assert_eq!(answer, 540212);
    }

    #[test]
    fn part_2_test() {
        let input = get_input_file(3, "test.txt").unwrap();
        let answer = part_2(input);
        assert_eq!(answer, 467835);
    }

    #[test]
    fn part_2_actual() {
        let input = get_input_file(3, "input.txt").unwrap();
        let answer = part_2(input);
        assert_eq!(answer, 87605697);
    }
}
