mod test_digits;
mod high_and_low;
mod pin_code;
mod tower_builder;
mod high_word_score;
mod convert_string_to_camel_case;
mod create_phone_number;
mod pangram;
mod observed_pin;
mod order_please;
mod sum_of_pairs;
mod multiplying_numbers_as_strings;
mod snail_sort;
mod sudoku;

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
