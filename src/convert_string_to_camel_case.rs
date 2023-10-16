use std::collections::HashMap;
use once_cell::sync::Lazy;

fn test_convert_string(input : &str) -> String
{
    let mut words: Vec<String> = vec![];
    for (i, word) in input.split(&['-', '_']).enumerate()
    {
        let mut converted_word = "".to_owned();
        for (ic, ch) in word.chars().enumerate()
        {
            if i == 0
            {
                converted_word.push(ch);
            }
            else 
            {
                if ic == 0
                {
                    converted_word.push_str(&ch.to_uppercase().to_string())
                }
                else 
                {
                    converted_word.push(ch);
                }
            }
        }
        words.push(converted_word);
    }
    words.join("")
}



#[cfg(test)]
mod tests
{   
    #[test]
    fn test_builder()
    {
        let f = super::test_convert_string("one_two_three");
        println!("{}", f)
      
    }
}