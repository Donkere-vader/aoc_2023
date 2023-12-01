use common::get_input_file;
use day_1::part_2;

fn main() {
    let input = get_input_file(1, "input.txt").unwrap();
    let answer = part_2(input);
    println!("answer: {answer}");
}
