use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Eq)]
struct Sorter<'a>
{
    text: &'a str
}

impl<'a> Sorter<'a>
{
    pub fn new(text: &'a str) -> Self
    {
        Sorter { text }
    }
    pub fn get_text(&self) -> &str
    {
        self.text
    } 
    pub fn get_digit(&self) -> u32
    {
        let mut number: Option<u32> = None;
        for ch in self.text.chars()
        {
            if ch.is_digit(10)
            {
                number = ch.to_digit(10);
            }
        }
        number.unwrap_or(0)
    }
}

impl<'a> PartialEq for Sorter<'a>
{
    fn eq(&self, other: &Self) -> bool 
    {
        self.get_digit() == other.get_digit()
    }
}

impl<'a> PartialOrd for Sorter<'a>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> 
    {
        Some(self.get_digit().cmp(&other.get_digit()))
    }
}

impl<'a> Ord for Sorter<'a>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering 
    {
        self.get_digit().cmp(&other.get_digit())
    }
}

fn order(sentence: &str) -> String 
{
    let mut ws: Vec<_> = sentence.split_whitespace().map(String::from).collect();
    ws.sort_by_key(|s| s.chars().find(|c| c.is_digit(10)).unwrap());
    ws.join(" ")
}



#[cfg(test)]
mod tests
{
    use crate::order_please::Sorter;
   
    #[test]
    fn test_builder()
    {
        let string = "is2 Thi1s T4est 3a";
        let mut sorter = string.split_whitespace().map(|m| Sorter::new(m)).collect::<Vec<Sorter>>();
        sorter.sort();
        let string = sorter.iter().map(|m| m.get_text()).collect::<Vec<&str>>().join(" ");
        assert_eq!(string, "Thi1s is2 3a T4est");
        //println!("{}", f)
      
    }
}