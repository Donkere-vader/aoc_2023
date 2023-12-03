use common::get_input_file;
use day_2::part_2;

fn main() {
    let input = get_input_file(2, "input.txt").unwrap();
    let answer = part_2(input);
    println!("answer: {answer}");
}
