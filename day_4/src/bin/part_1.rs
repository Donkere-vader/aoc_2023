use common::get_input_file;
use day_4::part_1;

fn main() {
    let input = get_input_file(4, "input.txt").unwrap();
    let answer = part_1(input);
    println!("answer: {answer}");
}
