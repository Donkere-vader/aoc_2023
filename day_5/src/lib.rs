
pub fn part_1(input: String) -> usize {
    0
}

pub fn part_2(input: String) -> usize {
    0
}

#[cfg(test)]
mod test {
    use common::get_input_file;

    use super::{part_1, part_2};

    #[test]
    fn part_1_test() {
        let input = get_input_file(5, "test.txt").unwrap();
        let answer = part_1(input);
        assert_eq!(answer, 0);
    }
}
