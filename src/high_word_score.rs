use std::collections::HashMap;
use once_cell::sync::Lazy;


pub static SCORE: Lazy<HashMap<char, usize>> = Lazy::new(|| HashMap::from(
    [
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
    ]
));

fn test_words_score(input : &str) -> &str
{
    let mut high_score: Vec<(usize, usize, &str)> = vec![];
    for (i, word) in input.split(" ").enumerate()
    {
        let mut current_scrore = 0usize;
        for ch in word.chars()
        {
            current_scrore += SCORE.get(&ch).unwrap_or(&0usize);
        }
        high_score.push((i, current_scrore, word));
    }
    high_score.reverse();
    let hs = high_score
    .iter()
    .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    .unwrap();
    println!("{}:{}", hs.0, hs.1);
    hs.2
}

#[cfg(test)]
mod tests
{   
    #[test]
    fn test_builder()
    {
        let f = super::test_words_score("what time are we climbing up the volcano");
        println!("{}", f)
      
    }
}