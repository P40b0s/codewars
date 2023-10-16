use std::{collections::HashMap, fmt::{format, Display}};
use once_cell::sync::Lazy;

fn create_phone_number(number: &[u8]) -> String
{
    let prefix = to_str(&number[0..=2]);
    let body_first = to_str(&number[3..=5]);
    let body_last = to_str(&number[6..]);
    format!("({}) {}-{}", prefix, body_first, body_last)
} // returns "(123) 456-7890"

fn to_str(slice: &[u8]) -> String
{
    slice.iter()
    .map(|m| m.to_string())
    .collect::<Vec<String>>()
    .join("")
}



#[cfg(test)]
mod tests
{   
    #[test]
    fn test_builder()
    {
        let f = super::create_phone_number(&[1,2,3,4,5,6,7,8,9]);
        println!("{}", f)
      
    }
}