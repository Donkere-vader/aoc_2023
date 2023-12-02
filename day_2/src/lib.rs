use std::collections::HashMap;

use once_cell::sync::OnceCell;
use regex::Regex;

#[derive(PartialEq, Eq, Hash, Debug)]
enum Color {
    Blue,
    Red,
    Green,
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        match value {
            "blue" => Color::Blue,
            "red" => Color::Red,
            "green" => Color::Green,
            _ => panic!("unknown color: {value}"),
        }
    }
}

#[derive(Debug)]
struct Game {
    pub num: usize,
    pub cube_sets: Vec<Vec<(usize, Color)>>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        static GAME_NUM_RE: OnceCell<Regex> = OnceCell::new();
        GAME_NUM_RE.get_or_init(|| {
            Regex::new("Game ([0-9]+):").unwrap()
        });
        static CUBE_AMOUNT_RE: OnceCell<Regex> = OnceCell::new();
        CUBE_AMOUNT_RE.get_or_init(|| {
            Regex::new("([0-9]+)\\s*(blue|green|red)").unwrap()
        });

        let game_num = GAME_NUM_RE.get().unwrap().captures_iter(value).next().unwrap()[1]
            .parse::<usize>()
            .unwrap();
        
        let mut cube_sets = Vec::new();

        for string_cube_set in value.split(";") {
            let cubes = CUBE_AMOUNT_RE.get().unwrap().captures_iter(string_cube_set)
                .map(|c| (c[1].parse::<usize>().unwrap(), Color::from(&c[2])))
                .collect::<Vec<(usize, Color)>>();

            cube_sets.push(cubes);
        }
    
        Self {
            num: game_num,
            cube_sets,
        }
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    input.split("\n").into_iter()
        .filter(|l| l.len() > 0)
        .map(Game::from).collect()
}

fn highest_occurrence_of_color_in_game(game: &Game, color: &Color) -> usize {
    game.cube_sets
        .iter()
        .map(|s| s.iter()
            .filter(|c| c.1 == *color)
            .map(|c| c.0)
            .max()
            .unwrap_or(0))
        .max()
        .unwrap()
}

pub fn part_1(input: String) -> usize {
    let games: Vec<Game> = parse_input(&input);
    let mut total = 0;

    for game in games.iter() {
        let mut cube_occurrence = HashMap::new();

        for color in vec![Color::Red, Color::Blue, Color::Green] {
            let cube_color_occurrence = highest_occurrence_of_color_in_game(game, &color);
            cube_occurrence.insert(color, cube_color_occurrence);
        }

        if cube_occurrence[&Color::Red] <= 12
                && cube_occurrence[&Color::Green] <= 13
                && cube_occurrence[&Color::Blue] <= 14 {
            total += game.num;
        }
    }

    total
}

pub fn part_2(input: String) -> usize {
    let games: Vec<Game> = parse_input(&input);
    let mut total = 0;

    for game in games.iter() {
        let mut game_score = 1;

        for color in vec![Color::Red, Color::Blue, Color::Green] {
            let cube_color_occurrence = highest_occurrence_of_color_in_game(game, &color);
            game_score *= cube_color_occurrence;
        }

        total += game_score;
    }

    total
}

#[cfg(test)]
mod day_2_tests {
    use common::get_input_file;

    use crate::{part_1, part_2};

    #[test]
    fn part_1_test() {
        let input = get_input_file(2, "test.txt").unwrap();
        let answer = part_1(input);
        assert_eq!(answer, 8);
    }

    #[test]
    fn part_1_actual() {
        let input = get_input_file(2, "input.txt").unwrap();
        let answer = part_1(input);
        assert_eq!(answer, 3099);
    }

    #[test]
    fn part_2_test() {
        let input = get_input_file(2, "test.txt").unwrap();
        let answer = part_2(input);
        assert_eq!(answer, 2286);
    }

    #[test]
    fn part_2_actual() {
        let input = get_input_file(2, "input.txt").unwrap();
        let answer = part_2(input);
        assert_eq!(answer, 72970);
    }
}
