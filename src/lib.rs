mod test_digits;
mod high_and_low;
mod pin_code;
mod tower_builder;
mod high_word_score;
mod convert_string_to_camel_case;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
