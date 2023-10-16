use std::{collections::HashMap, fmt::{format, Display}};
use once_cell::sync::Lazy;
pub static SCHEME: Lazy<HashMap<char, Vec<u8>>> = Lazy::new(|| HashMap::from(
    [
        ('1', vec![1, 2, 4]),
        ('2', vec![1, 2, 3, 5]),
        ('3', vec![2, 3, 6]),
        ('4', vec![1, 4, 5, 7]),
        ('5', vec![2, 4, 5, 6, 8]),
        ('6', vec![3, 5, 6, 9]),
        ('7', vec![4, 7, 8]),
        ('8', vec![0, 5, 7, 8, 9]),
        ('9', vec![6, 8, 9]),
        ('0', vec![0, 8]),
    ]
));

//2,3,6
//3,5,6,9
//6,8,9

//"Pack my box with five dozen liquor jugs."
fn pin(observed: &str) -> bool
{
    
    //завичсит от длинны пина
   
    let mut contains : Vec<String> = vec![];
    let pin_len = observed.len();
    for obs in observed.chars()
    {
        
    }
    // let rnd = range.filter(|f|
    // {
    //     let f = f.to_string();
        
    //     f.contains("1") && f.len() == pin_len
    // }).collect::<Vec<i32>>();
    return false;

}

fn search_range(contains: &Vec<u8>, pin_len: usize)
{
    let range = 0..9999;
    let rnd = range.filter(|f|
    {
        let f = f.to_string();
        
        f.contains("1") && f.len() == pin_len
    }).collect::<Vec<i32>>();
}



#[cfg(test)]
mod tests
{   
    #[test]
    fn test_range()
    {
        let num = 369;
        let mut all: Vec<u8> = vec![];
        let v1 = vec![2u8,3,6];
        let v2 = vec![3u8,5,6,9];
        let v3 = vec![6u8,8,9];
        all.extend(v1);
        all.extend(v2);
        all.extend(v3);
        let all = all.into_iter()
        .collect::<std::collections::HashSet<u8>>()
        .into_iter()
        .collect::<Vec<u8>>();
        let mut result : Vec<String>;
        
        //2,3,6
//3,5,6,9
//6,8,9
        println!("{:?}", all);
      
    }
}