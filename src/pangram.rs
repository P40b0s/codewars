use std::{collections::HashMap, fmt::{format, Display}};
use once_cell::sync::Lazy;

//"Pack my box with five dozen liquor jugs."
fn pangram(sent: &str) -> bool
{
    let abc_len = 26;
    let mut hm = HashMap::new();
    for ch in sent.to_lowercase().chars()
    {
        if ch.is_ascii_alphabetic()
        {
            hm.insert(ch, ch);
        }
    }
    hm.len() == abc_len
} // returns "(123) 456-7890"



#[cfg(test)]
mod tests
{   
    #[test]
    fn test_builder()
    {
        let f = super::pangram("Pack my box with five dozen liquor jugs.");
        println!("{}", f)
      
    }
}