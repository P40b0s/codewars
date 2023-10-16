

fn high_and_low(numbers: &str) -> String
{
   let mut res = numbers
   .split(" ")
   .map(|m| m.parse::<i32>().unwrap_or(0))
   .collect::<Vec<i32>>();
   res.sort();
   [res[res.len() -1].to_string(), " ".to_owned(), res[0].to_string()].concat()
}


#[cfg(test)]
mod tests
{   
    #[test]
    fn test_high_and_low()
    {
        println!("{}", super::high_and_low("5 2 -3 0 9"));
    }
}